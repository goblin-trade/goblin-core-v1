use alloc::vec::Vec;
use stylus_sdk::alloy_primitives::{Address, FixedBytes};

use crate::{
    program::{try_withdraw, FailedToReduce, GoblinError, GoblinResult, PricesNotInOrder},
    quantities::{BaseAtomsRaw, BaseLots, QuoteAtomsRaw, QuoteLots, Ticks, WrapperU64},
    require,
    state::{
        AskOrderId, BidOrderId, BitmapGroup, GroupPosition, MarketState, MatchingEngineResponse,
        OrderId, OuterIndex, ReduceOrderInnerResponse, RestingOrderIndex,
        RestingOrderSearcherAndRemover, Side, SlotActions, SlotRestingOrder, SlotStorage,
        TickIndices, TraderState,
    },
    GoblinMarket,
};

pub struct ReduceOrderPacket {
    // ID of order to reduce
    pub order_id: OrderId,

    // Reduce at most these many lots. Pass u64::MAX to close
    pub lots_to_remove: BaseLots,

    // Revert entire TX if reduction fails for this order
    pub revert_if_fail: bool,
}

impl From<&FixedBytes<17>> for ReduceOrderPacket {
    fn from(bytes: &FixedBytes<17>) -> Self {
        ReduceOrderPacket {
            order_id: OrderId {
                price_in_ticks: Ticks::new(u64::from_be_bytes(bytes[0..8].try_into().unwrap())),
                resting_order_index: RestingOrderIndex::new(bytes[8]),
            },
            lots_to_remove: BaseLots::new(u64::from_be_bytes(bytes[9..16].try_into().unwrap())),
            revert_if_fail: (bytes[16] & 0b0000_0001) != 0,
        }
    }
}

/// Try to reduce or cancel one or more resting orders. Pass MAX amount to cancel an order.
///
/// # Arguments
///
/// * `context`
/// * `trader` - Reduce orders belonging to this trader
/// * `order_packets`
/// * `claim_funds` - Whether to claim ERC20 tokens to wallet, or whether to credit
/// them to trader state
///
pub fn process_reduce_multiple_orders(
    context: &mut GoblinMarket,
    trader: Address,
    order_packets: Vec<FixedBytes<17>>,
    claim_funds: bool,
) -> GoblinResult<()> {
    // Read
    let slot_storage = &mut SlotStorage::new();
    let market_state = &mut MarketState::read_from_slot(slot_storage);
    let trader_state = &mut TraderState::read_from_slot(slot_storage, trader);

    // Mutate
    // State reads and writes are performed inside reduce_multiple_orders_inner()
    // The number of slot reads is dynamic
    let MatchingEngineResponse {
        num_quote_lots_out,
        num_base_lots_out,
        ..
    } = reduce_multiple_orders_inner(
        slot_storage,
        market_state,
        trader,
        trader_state,
        order_packets,
        claim_funds,
    )?;

    // Write state
    market_state.write_to_slot(slot_storage)?;
    trader_state.write_to_slot(slot_storage, trader);
    SlotStorage::storage_flush_cache(true);

    // Transfer tokens
    if claim_funds {
        let quote_amount_raw = QuoteAtomsRaw::from_lots(num_quote_lots_out);
        let base_amount_raw = BaseAtomsRaw::from_lots(num_base_lots_out);
        try_withdraw(context, quote_amount_raw, base_amount_raw, trader)?;
    }

    Ok(())
}

/// Try to reduce multiple orders by ID
///
/// It is possible that an order ID is already closed, and also occupied by
/// another trader. The current behavior is that if one reduction fails,
/// continue trying to reduction others.
///
/// Order IDs should be grouped by outer_ids and by side for efficiency.
///
/// Reduction involves
///
/// - Updating trader state
/// - Updating / closing the order slot
/// - Updating the bitmap
/// - Removing the outer index from index list if the outer index is closed
/// - Updating outer index sizes and best prices in market state
///
/// Opportunity to use VM cache is limited to bitmap group. We need order IDs in
/// correct order for index list updations
///
/// # Arguments
///
/// * `slot_storage`
/// * `market_state`
/// * `trader`
/// * `trader_state`
/// * `order_packets`
/// * `claim_funds`
///
pub fn reduce_multiple_orders_inner(
    slot_storage: &mut SlotStorage,
    market_state: &mut MarketState,
    trader: Address,
    trader_state: &mut TraderState,
    order_packets: Vec<FixedBytes<17>>,
    claim_funds: bool,
) -> GoblinResult<MatchingEngineResponse> {
    let mut quote_lots_released = QuoteLots::ZERO;
    let mut base_lots_released = BaseLots::ZERO;

    let mut cached_bitmap_group: Option<(BitmapGroup, OuterIndex)> = None;

    let mut bid_index_list = market_state.get_index_list(Side::Bid);
    let mut ask_index_list = market_state.get_index_list(Side::Ask);
    let mut order_id_checker = OrderIdChecker::new();
    let mut order_exists_checker = OrderExistsChecker::new(
        market_state.bids_outer_indices,
        market_state.asks_outer_indices,
    );
    // TODO ensure orders are in descending order for bids and in ascending order
    // for asks, i.e. moving away from the centre
    for order_packet_bytes in order_packets {
        let ReduceOrderPacket {
            order_id,
            lots_to_remove,
            revert_if_fail,
        } = ReduceOrderPacket::from(&order_packet_bytes);

        let TickIndices {
            outer_index,
            inner_index,
        } = order_id.price_in_ticks.to_indices();
        let side = order_id.side(market_state.best_bid_price, market_state.best_ask_price);

        // Ensure that successive order IDs move away from the center and that
        // duplicate values are not passed
        order_id_checker.check(order_id, side)?;

        let order_exists = order_exists_checker.order_exists(slot_storage, side, order_id);

        if order_exists {
            let mut resting_order = SlotRestingOrder::new_from_slot(slot_storage, order_id);

            if let Some(ReduceOrderInnerResponse {
                matching_engine_response,
                should_remove_order_from_book,
            }) = resting_order.reduce_order(
                trader_state,
                trader,
                &order_id,
                side,
                lots_to_remove,
                false,
                claim_funds,
            ) {
                quote_lots_released += matching_engine_response.num_quote_lots_out;
                base_lots_released += matching_engine_response.num_base_lots_out;

                if should_remove_order_from_book {
                    // TODO deactivate slot
                    // TODO update best market price
                } else {
                    resting_order.write_to_slot(slot_storage, &order_id)?;
                }
            } else if revert_if_fail {
                // None case- when order doesn't belong to trader
                return Err(GoblinError::FailedToReduce(FailedToReduce {}));
            }
        } else if revert_if_fail {
            // When order is not present
            return Err(GoblinError::FailedToReduce(FailedToReduce {}));
        }

        // let mut resting_order = SlotRestingOrder::new_from_slot(slot_storage, order_id);

        // if let Some(ReduceOrderInnerResponse {
        //     matching_engine_response,
        //     should_remove_order_from_book,
        // }) = resting_order.reduce_order(
        //     trader_state,
        //     trader,
        //     &order_id,
        //     side,
        //     lots_to_remove,
        //     false,
        //     claim_funds,
        // ) {
        //     quote_lots_released += matching_engine_response.num_quote_lots_out;
        //     base_lots_released += matching_engine_response.num_base_lots_out;

        //     // Order should be removed from the book. Flip its corresponding bitmap.
        //     // No need to write cleared resting order to slot
        //     if should_remove_order_from_book {
        //         // SLOAD and cache the bitmap group. This saves us from duplicate SLOADs in future
        //         // Read a new bitmap group if no cache exists or if the outer index does not match
        //         if cached_bitmap_group.is_none() || cached_bitmap_group.unwrap().1 != outer_index {
        //             // Before reading a new bitmap group, write the currently cached one to slot
        //             if let Some((old_bitmap_group, old_outer_index)) = cached_bitmap_group {
        //                 old_bitmap_group.write_to_slot(slot_storage, &old_outer_index);
        //             }

        //             // Read new
        //             cached_bitmap_group = Some((
        //                 BitmapGroup::new_from_slot(slot_storage, outer_index),
        //                 outer_index,
        //             ));
        //         }

        //         let (mut bitmap_group, outer_index) = cached_bitmap_group.unwrap();
        //         let mut mutable_bitmap = bitmap_group.get_bitmap_mut(&inner_index);
        //         mutable_bitmap.clear(&order_id.resting_order_index);

        //         // Remove outer index from index list if bitmap group is cleared
        //         // Outer indices of bitmap groups to be closed should be in descending order for bids and
        //         // in ascending order for asks.
        //         if !bitmap_group.is_active() {
        //             if side == Side::Bid {
        //                 bid_index_list.remove(slot_storage, outer_index)?;
        //             } else {
        //                 ask_index_list.remove(slot_storage, outer_index)?;
        //             }
        //         }
        //     } else {
        //         // Write reduced resting order to slot
        //         resting_order.write_to_slot(slot_storage, &order_id)?;
        //     }
        // } else if revert_if_fail {
        //     return Err(GoblinError::FailedToReduce(FailedToReduce {}));
        // }
    }

    // TODO write cached index list to slot
    // resting_order_remover.write_prepared_indices()

    // The last cached element is not written in the loop. It must be written at the end.
    if let Some((old_bitmap_group, old_outer_index)) = cached_bitmap_group {
        old_bitmap_group.write_to_slot(slot_storage, &old_outer_index);
    }

    bid_index_list.write_to_slot(slot_storage);
    ask_index_list.write_to_slot(slot_storage);

    // Update market state
    market_state.bids_outer_indices = bid_index_list.size;
    market_state.asks_outer_indices = ask_index_list.size;

    // TODO optimize- only run if one of the canceled orders had a price equal to the best price
    market_state.update_best_price(&bid_index_list, slot_storage);
    market_state.update_best_price(&ask_index_list, slot_storage);

    Ok(MatchingEngineResponse::new_withdraw(
        base_lots_released,
        quote_lots_released,
    ))
}

/// Ensures that successive order ids to remove are in correct order
///
/// Successive IDs must be in ascending order for asks and in descending order for bids
struct OrderIdChecker {
    last_bid_order_id: Option<BidOrderId>,
    last_ask_order_id: Option<AskOrderId>,
}

impl OrderIdChecker {
    pub fn new() -> Self {
        OrderIdChecker {
            last_bid_order_id: None,
            last_ask_order_id: None,
        }
    }

    pub fn check(&mut self, order_id: OrderId, side: Side) -> GoblinResult<()> {
        if side == Side::Bid && self.last_bid_order_id.is_some() {
            let bid_order_id = BidOrderId { inner: order_id };
            require!(
                bid_order_id < self.last_bid_order_id.unwrap(),
                GoblinError::PricesNotInOrder(PricesNotInOrder {})
            );
            self.last_bid_order_id = Some(bid_order_id);
        }
        if side == Side::Ask && self.last_ask_order_id.is_some() {
            let ask_order_id = AskOrderId { inner: order_id };
            require!(
                ask_order_id < self.last_ask_order_id.unwrap(),
                GoblinError::PricesNotInOrder(PricesNotInOrder {})
            );
            self.last_ask_order_id = Some(ask_order_id);
        }

        Ok(())
    }
}

struct OrderExistsChecker {
    bid_bitmap_reader: RestingOrderSearcherAndRemover,
    ask_bitmap_reader: RestingOrderSearcherAndRemover,
}

impl OrderExistsChecker {
    pub fn new(bids_outer_indices: u16, asks_outer_indices: u16) -> Self {
        OrderExistsChecker {
            bid_bitmap_reader: RestingOrderSearcherAndRemover::new(bids_outer_indices, Side::Bid),
            ask_bitmap_reader: RestingOrderSearcherAndRemover::new(asks_outer_indices, Side::Ask),
        }
    }

    fn reader(&mut self, side: Side) -> &mut RestingOrderSearcherAndRemover {
        match side {
            Side::Bid => &mut self.bid_bitmap_reader,
            Side::Ask => &mut self.ask_bitmap_reader,
        }
    }

    pub fn order_exists(
        &mut self,
        slot_storage: &mut SlotStorage,
        side: Side,
        order_id: OrderId,
    ) -> bool {
        self.reader(side).order_present(slot_storage, order_id)
    }

    pub fn remove_order(
        &mut self,
        slot_storage: &mut SlotStorage,
        side: Side,
        order_id: OrderId,
        // group_position: GroupPosition,
    ) {
        let group_position = GroupPosition::from(&order_id);
        self.reader(side).deactivate_in_current(group_position);
    }
}

fn remove_order_from_book(
    reader: &mut RestingOrderSearcherAndRemover,
    slot_storage: &mut SlotStorage,
    order_id: OrderId,
) {
    let group_position = GroupPosition::from(&order_id);

    // Since the order was looked up before, we're already on the correct bitmap group and outer index
    // Deactivate at current group position
    reader.deactivate_in_current(group_position);

    // If deactivated order was the outermost order
    // - Find next active bit in all groups
    // - Update market price
}
