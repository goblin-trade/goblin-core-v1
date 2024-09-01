use alloc::vec::Vec;
use stylus_sdk::alloy_primitives::{Address, FixedBytes};

use crate::{
    program::{try_withdraw, FailedToReduce, GoblinError, GoblinResult},
    quantities::{BaseAtomsRaw, BaseLots, QuoteAtomsRaw, QuoteLots, Ticks, WrapperU64},
    state::{
        BitmapGroup, MarketState, MatchingEngineResponse, OrderId, OuterIndex,
        ReduceOrderInnerResponse, RestingOrderIndex, Side, SlotActions, SlotRestingOrder,
        SlotStorage, TickIndices, TraderState,
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

    for order_packet_bytes in order_packets {
        let ReduceOrderPacket {
            order_id,
            lots_to_remove: size,
            revert_if_fail,
        } = ReduceOrderPacket::from(&order_packet_bytes);

        let side = order_id.side(market_state.best_bid_price, market_state.best_ask_price);

        let mut resting_order = SlotRestingOrder::new_from_slot(slot_storage, order_id);

        if let Some(ReduceOrderInnerResponse {
            matching_engine_response,
            should_remove_order_from_book,
        }) = resting_order.reduce_order(
            trader_state,
            trader,
            &order_id,
            side.clone(),
            size,
            false,
            claim_funds,
        ) {
            resting_order.write_to_slot(slot_storage, &order_id)?;

            quote_lots_released += matching_engine_response.num_quote_lots_out;
            base_lots_released += matching_engine_response.num_base_lots_out;

            // Order should be removed from the book. Flip its corresponding bitmap.
            if should_remove_order_from_book {
                let TickIndices {
                    outer_index,
                    inner_index,
                } = order_id.price_in_ticks.to_indices();

                // SLOAD and cache the bitmap group. This saves us from duplicate SLOADs in future
                // Read a new bitmap group if no cache exists or if the outer index does not match
                if cached_bitmap_group.is_none() || cached_bitmap_group.unwrap().1 != outer_index {
                    // Before reading a new bitmap group, write the currently cached one to slot
                    if let Some((old_bitmap_group, old_outer_index)) = cached_bitmap_group {
                        old_bitmap_group.write_to_slot(slot_storage, &old_outer_index);
                    }

                    // Read new
                    cached_bitmap_group = Some((
                        BitmapGroup::new_from_slot(slot_storage, outer_index),
                        outer_index,
                    ));
                }

                let (mut bitmap_group, outer_index) = cached_bitmap_group.unwrap();
                let mut mutable_bitmap = bitmap_group.get_bitmap_mut(&inner_index);
                mutable_bitmap.clear(&order_id.resting_order_index);

                // Remove outer index from index list if bitmap group is cleared
                // Outer indices of bitmap groups to be closed should be in descending order for bids and
                // in ascending order for asks.
                if !bitmap_group.is_active() {
                    if side == Side::Bid {
                        bid_index_list.remove(slot_storage, outer_index)?;
                    } else {
                        ask_index_list.remove(slot_storage, outer_index)?;
                    }
                }
            }
        } else if revert_if_fail {
            return Err(GoblinError::FailedToReduce(FailedToReduce {}));
        }
    }

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
