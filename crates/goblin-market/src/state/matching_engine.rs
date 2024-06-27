use stylus_sdk::{
    alloy_primitives::{Address, B256},
    block,
};

use crate::{
    parameters::{BASE_LOTS_PER_BASE_UNIT, TICK_SIZE_IN_QUOTE_LOTS_PER_BASE_UNIT},
    program::{
        get_approved_base_lots, get_approved_quote_lots,
        new_order::{CondensedOrder, FailedMultipleLimitOrderBehavior},
        FailedToReduce, GoblinError, GoblinResult, PricesNotInOrder, ReduceOrderPacket,
    },
    quantities::{BaseLots, QuoteLots, Ticks, WrapperU64, MAX_TICK},
    require, GoblinMarket,
};

use super::{
    matching_engine_response, slot_storage, BitmapGroup, IndexList, InnerIndex, ListKey, ListSlot,
    MarketState, MatchingEngineResponse, OrderId, OrderPacket, OrderPacketMetadata, OuterIndex,
    RestingOrder, RestingOrderIndex, Side, SlotActions, SlotRestingOrder, SlotStorage, TickIndices,
    TraderState,
};
use alloc::{collections::btree_map::Range, vec::Vec};

pub struct MatchingEngine<'a> {
    pub slot_storage: &'a mut SlotStorage,
}

impl MatchingEngine<'_> {
    pub fn collect_fees(&mut self) -> GoblinResult<QuoteLots> {
        // Read
        let mut market = MarketState::read_from_slot(self.slot_storage);

        // Mutate
        let quote_lot_fees = market.unclaimed_quote_lot_fees;

        // Mark as claimed
        market.collected_quote_lot_fees += market.unclaimed_quote_lot_fees;
        market.unclaimed_quote_lot_fees = QuoteLots::ZERO;

        // Write
        market.write_to_slot(self.slot_storage)?;
        SlotStorage::storage_flush_cache(true);

        Ok(quote_lot_fees)
    }

    /// Try to claim the given number of lots from a trader's state.
    ///
    /// There is no eviction in Goblin.
    ///
    /// # Parameters
    ///
    /// * `trader` - The trader address
    /// * `num_quote_lots` - Number of lots to withdraw. Pass 0 if none should be withdrawn.
    /// Pass U64::MAX to withdraw all.
    /// * `num_base_lots` - Number of lots to withdraw. Pass 0 if none should be withdrawn.
    /// Pass U32::MAX to withdraw all. (max value of base_lots is U32::MAX)
    ///
    pub fn claim_funds(
        &mut self,
        trader: Address,
        num_quote_lots: QuoteLots,
        num_base_lots: BaseLots,
    ) -> MatchingEngineResponse {
        // Read
        let mut trader_state = TraderState::read_from_slot(self.slot_storage, trader);

        // Mutate
        let response = trader_state.claim_funds_inner(num_quote_lots, num_base_lots);

        // Write
        trader_state.write_to_slot(self.slot_storage, trader);
        SlotStorage::storage_flush_cache(true);

        response
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
    pub fn reduce_multiple_orders_inner(
        &mut self,
        market: &mut MarketState,
        trader: Address,
        order_packets: Vec<B256>,
        claim_funds: bool,
    ) -> GoblinResult<MatchingEngineResponse> {
        // Read state
        let mut trader_state = TraderState::read_from_slot(self.slot_storage, trader);

        let mut quote_lots_released = QuoteLots::ZERO;
        let mut base_lots_released = BaseLots::ZERO;

        let mut cached_bitmap_group: Option<(BitmapGroup, OuterIndex)> = None;

        let mut bid_index_list = market.get_index_list(Side::Bid);
        let mut ask_index_list = market.get_index_list(Side::Ask);

        for order_packet_bytes in order_packets {
            let ReduceOrderPacket {
                order_id,
                lots_to_remove: size,
                revert_if_fail,
            } = ReduceOrderPacket::decode(&order_packet_bytes.0);

            let side = order_id.side(market.best_bid_price, market.best_ask_price);

            let mut order = SlotRestingOrder::new_from_slot(self.slot_storage, order_id);

            if let Some(ReduceOrderInnerResponse {
                matching_engine_response,
                should_remove_order_from_book,
            }) = order.reduce_order(
                &mut trader_state,
                trader,
                &order_id,
                side.clone(),
                size,
                false,
                claim_funds,
            ) {
                order.write_to_slot(self.slot_storage, &order_id)?;

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
                    if cached_bitmap_group.is_none()
                        || cached_bitmap_group.unwrap().1 != outer_index
                    {
                        // Before reading a new bitmap group, write the currently cached one to slot
                        if let Some((old_bitmap_group, old_outer_index)) = cached_bitmap_group {
                            old_bitmap_group.write_to_slot(self.slot_storage, &old_outer_index);
                        }

                        // Read new
                        cached_bitmap_group = Some((
                            BitmapGroup::new_from_slot(self.slot_storage, outer_index),
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
                            bid_index_list.remove(self.slot_storage, outer_index)?;
                        } else {
                            ask_index_list.remove(self.slot_storage, outer_index)?;
                        }
                    }
                }
            } else if revert_if_fail {
                return Err(GoblinError::FailedToReduce(FailedToReduce {}));
            }
        }

        // The last cached element is not written in the loop. It must be written at the end.
        if let Some((old_bitmap_group, old_outer_index)) = cached_bitmap_group {
            old_bitmap_group.write_to_slot(self.slot_storage, &old_outer_index);
        }

        bid_index_list.write_to_slot(self.slot_storage);
        ask_index_list.write_to_slot(self.slot_storage);

        // Update market state
        market.bids_outer_indices = bid_index_list.size;
        market.asks_outer_indices = ask_index_list.size;

        // TODO optimize- only run if one of the canceled orders had a price equal to the best price
        market.update_best_price(&bid_index_list, self.slot_storage);
        market.update_best_price(&ask_index_list, self.slot_storage);

        // write trader state
        trader_state.write_to_slot(self.slot_storage, trader);

        Ok(MatchingEngineResponse::new_withdraw(
            base_lots_released,
            quote_lots_released,
        ))
    }

    pub fn place_multiple_new_orders(
        &mut self,
        context: &mut GoblinMarket,
        trader: Address,
        to: Address,
        failed_multiple_limit_order_behavior: FailedMultipleLimitOrderBehavior,
        bids: Vec<B256>,
        asks: Vec<B256>,
        client_order_id: u128,
        no_deposit: bool,
    ) -> GoblinResult<MatchingEngineResponse> {
        // Read states
        let mut market = MarketState::read_from_slot(self.slot_storage);
        let mut trader_state = TraderState::read_from_slot(self.slot_storage, trader);

        let mut quote_lots_to_deposit = QuoteLots::ZERO;
        let mut base_lots_to_deposit = BaseLots::ZERO;

        // Read quote and base lots available with trader
        // free tokens + balance with trader
        // Optimization- don't read balanceOf() beforehand
        // Check balances if the requirement exceeds existing deposit and no_deposit is false
        let mut base_lots_available = trader_state.base_lots_free;
        let mut quote_lots_available = trader_state.quote_lots_free;
        // need to add approved balance not total balance

        // Track whether token allowances have been read
        let mut base_allowance_read = false;
        let mut quote_allowance_read = false;

        // orders at centre of the book are placed first, then move away.
        // bids- descending order
        // asks- ascending order
        for (book_orders, side, mut last_price) in [
            (&bids, Side::Bid, Ticks::new(MAX_TICK)),
            (&asks, Side::Ask, Ticks::new(0)),
        ]
        .iter()
        {
            for order_bytes in *book_orders {
                let condensed_order = CondensedOrder::decode(&order_bytes.0);

                // Ensure orders are in correct order- descending for bids and ascending for asks
                if *side == Side::Bid {
                    require!(
                        condensed_order.price_in_ticks < last_price,
                        GoblinError::PricesNotInOrder(PricesNotInOrder {})
                    );
                } else {
                    require!(
                        condensed_order.price_in_ticks > last_price,
                        GoblinError::PricesNotInOrder(PricesNotInOrder {})
                    );

                    // Price can't exceed max
                    require!(
                        condensed_order.price_in_ticks <= Ticks::new(MAX_TICK),
                        GoblinError::PricesNotInOrder(PricesNotInOrder {})
                    );
                }

                let order_packet = OrderPacket::PostOnly {
                    side: *side,
                    price_in_ticks: condensed_order.price_in_ticks,
                    num_base_lots: condensed_order.size_in_base_lots,
                    client_order_id,
                    reject_post_only: failed_multiple_limit_order_behavior.should_fail_on_cross(),
                    use_only_deposited_funds: no_deposit,
                    track_block: condensed_order.track_block,
                    last_valid_block_or_unix_timestamp_in_seconds: condensed_order
                        .last_valid_block_or_unix_timestamp_in_seconds,
                    fail_silently_on_insufficient_funds: failed_multiple_limit_order_behavior
                        .should_skip_orders_with_insufficient_funds(),
                };

                let matching_engine_response = {
                    if failed_multiple_limit_order_behavior
                        .should_skip_orders_with_insufficient_funds()
                        && !order_packet_has_sufficient_funds(
                            context,
                            &order_packet,
                            trader,
                            &mut base_lots_available,
                            &mut quote_lots_available,
                            &mut base_allowance_read,
                            &mut quote_allowance_read,
                        )
                    {
                        // Skip this order if the trader does not have sufficient funds
                        continue;
                    }

                    // matching_engine_response gives the number of tokens required
                    // these are added and then compared in the end

                    // TODO call place_order()
                    MatchingEngineResponse::default()
                };

                // finally set last price
                last_price = condensed_order.price_in_ticks;
            }
        }

        Ok(MatchingEngineResponse::default())
    }

    fn place_order_inner(
        &mut self,
        market_state: &mut MarketState,
        trader_state: &mut TraderState,
        trader: Address,
        to: Address,
        order_packet: &mut OrderPacket,
    ) -> Option<MatchingEngineResponse> {
        let side = order_packet.side();

        match side {
            Side::Bid => {
                if order_packet.get_price_in_ticks() == Ticks::ZERO {
                    return None;
                }
            }
            Side::Ask => {
                if !order_packet.is_take_only() {
                    let tick_price = order_packet.get_price_in_ticks();
                    order_packet.set_price_in_ticks(tick_price.max(Ticks::ONE));
                }
            }
        }

        if order_packet.num_base_lots() == 0 && order_packet.num_quote_lots() == 0 {
            // Either num_base_lots or num_quote_lots must be nonzero
            return None;
        }

        // For IOC order types exactly one of num_quote_lots or num_base_lots needs to be specified.
        if let OrderPacket::ImmediateOrCancel {
            num_base_lots,
            num_quote_lots,
            ..
        } = *order_packet
        {
            if num_base_lots > BaseLots::ZERO && num_quote_lots > QuoteLots::ZERO
                || num_base_lots == BaseLots::ZERO && num_quote_lots == QuoteLots::ZERO
            {
                return None;
            }
        }

        let current_block = block::number() as u32;
        let current_unix_timestamp = block::timestamp() as u32;

        if order_packet.is_expired(current_block, current_unix_timestamp) {
            // Do not fail the transaction if the order is expired, but do not place or match the order
            return Some(MatchingEngineResponse::default());
        }

        let mut index_list = market_state.get_index_list(side);

        // Build resting_order and matching_engine_response
        if let OrderPacket::PostOnly {
            price_in_ticks,
            reject_post_only,
            ..
        } = order_packet
        {
            // Handle cases where PostOnly order would cross the book
            // self.check_for_cross(
            //     market_state,
            //     &mut index_list,
            //     *price_in_ticks,
            //     current_block,
            //     current_unix_timestamp,
            // );
        };

        // if let OrderPacket::PostOnly {
        //     price_in_ticks,
        //     reject_post_only,
        //     ..
        // } = &mut order_packet
        // {
        //     // Handle cases where PostOnly order would cross the book
        //     // self.check_for_cross(
        //     //     side,
        //     //     *price_in_ticks,
        //     //     current_block,
        //     //     current_unix_timestamp,
        //     // );
        // };

        None
    }

    /// This function determines whether a PostOnly order crosses the book.
    /// If the order crosses the book, the function returns the ID of the best unexpired
    /// crossing order (price, index) on the opposite side of the book. Otherwise, it returns None.
    ///
    /// The function closes all expired orders till an unexpired order is found.
    /// Design difference- unlike Phoenix, this function can close expired orders that do not cross.
    ///
    /// # Arguments
    ///
    /// * `slot_storage`
    /// * `market_state`
    /// * `side`
    /// * `num_ticks`
    /// * `current_block`
    /// * `current_unix_timestamp_in_seconds`
    ///
    fn check_for_cross(
        &mut self,
        slot_storage: &mut SlotStorage,
        market_state: &mut MarketState,
        side: Side,
        num_ticks: Ticks,
        current_block: u32,
        current_unix_timestamp_in_seconds: u32,
    ) -> Option<OrderId> {
        // Index list on the opposite side
        let mut index_list = market_state.get_index_list(side.opposite());
        let opposite_best_price = market_state.best_price(side.opposite());

        if index_list.size == 0 // Book empty case
            // No cross case
            || (side == Side::Bid && num_ticks < opposite_best_price)
            || (side == Side::Ask && num_ticks > opposite_best_price)
        {
            return None;
        }
        // Now there is atleast one order that crosses

        // Avoid duplicate SLOADs for a values with the same slot index
        let mut cached_list_slot: Option<ListSlot> = None;

        for i in (0..index_list.size).rev() {
            let slot_index = i / 16;
            let relative_index = i as usize % 16;

            // Load list slot if this is the first iteration or if values in the cached slot are
            // completely read
            if cached_list_slot.is_none() || relative_index == 15 {
                let list_slot_key = ListKey { index: slot_index };
                cached_list_slot = Some(ListSlot::new_from_slot(slot_storage, list_slot_key));
            }

            // Read bitmap group for each outer index in the list slot
            let outer_index = cached_list_slot.unwrap().get(relative_index);
            let mut bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);

            // If this is the first bitmap group being read, start from the inner index of the cached
            // best opposite price
            let previous_inner_index = if i == 0 {
                Some(opposite_best_price.inner_index())
            } else {
                None
            };

            // 2. loop through bitmaps in the group
            for j in bitmap_group.bitmap_iterator(index_list.side, previous_inner_index) {
                let inner_index = InnerIndex::new(j);
                let mut bitmap = bitmap_group.get_bitmap_mut(&inner_index);

                if !bitmap.empty() {
                    let current_price = Ticks::from_indices(outer_index, inner_index);

                    let mut bits = *bitmap.inner;
                    for k in 0..8 {
                        // Read smallest bit
                        let bit = bits & 1;

                        if bit != 0 {
                            let resting_order_index = RestingOrderIndex::new(k);

                            // Resting order present
                            let order_id = OrderId {
                                price_in_ticks: current_price,
                                resting_order_index,
                            };
                            let order = SlotRestingOrder::new_from_slot(slot_storage, order_id);

                            // Remove expired order
                            if order.expired(current_block, current_unix_timestamp_in_seconds) {
                                let trader_state = &mut TraderState::read_from_slot(
                                    &slot_storage,
                                    order.trader_address,
                                );

                                let mut order = SlotRestingOrder::new_from_slot(slot_storage, order_id);
                                order.reduce_order(
                                    trader_state,
                                    order.trader_address,
                                    &order_id,
                                    side.opposite(),
                                    BaseLots::MAX,
                                    true,
                                    false,
                                )?;
                                order.write_to_slot(slot_storage, &order_id).ok();

                                trader_state.write_to_slot(self.slot_storage, order.trader_address);
                                bitmap.clear(&resting_order_index);
                            } else {
                                // Best unexpired order found. Write states and exit function
                                // Since an active order is present at the current outer index,
                                // do not remove it from the index list.
                                bitmap_group.write_to_slot(slot_storage, &outer_index);
                                index_list.write_to_slot(slot_storage);

                                return if (side.opposite() == Side::Bid
                                    && current_price >= num_ticks)
                                    || (side.opposite() == Side::Ask && current_price <= num_ticks)
                                {
                                    // Unexpired crossing order
                                    Some(order_id)
                                } else {
                                    // Non-crossing order
                                    None
                                };
                            }
                        }
                        // Right shift
                        bits >>= 1;
                    }
                }
            }
            // Entire group traversed. The group only had expired orders which were removed
            // Write the empty bitmap group and remove the outer index from index list
            bitmap_group.write_to_slot(slot_storage, &outer_index);
            index_list.remove(slot_storage, outer_index).ok();
        }
        // Reached if all orders were expired
        index_list.write_to_slot(slot_storage);
        return None;
    }
}

pub struct ReduceOrderInnerResponse {
    pub matching_engine_response: MatchingEngineResponse,
    pub should_remove_order_from_book: bool,
}

fn order_packet_has_sufficient_funds(
    context: &GoblinMarket,
    order_packet: &OrderPacket,
    trader: Address,
    base_lots_available: &mut BaseLots,
    quote_lots_available: &mut QuoteLots,
    base_allowance_read: &mut bool,
    quote_allowance_read: &mut bool,
) -> bool {
    match order_packet.side() {
        Side::Ask => {
            if *base_lots_available < order_packet.num_base_lots() {
                // Lazy load available approved balance for base token
                if !*base_allowance_read {
                    *base_lots_available += get_approved_base_lots(context, trader);
                    *base_allowance_read = true;
                }

                return *base_lots_available >= order_packet.num_base_lots();
            }
        }
        Side::Bid => {
            let quote_lots_required = order_packet.get_price_in_ticks()
                * TICK_SIZE_IN_QUOTE_LOTS_PER_BASE_UNIT
                * order_packet.num_base_lots()
                / BASE_LOTS_PER_BASE_UNIT;

            if *quote_lots_available < quote_lots_required {
                // Lazy load available approved balance for quote token
                if !*quote_allowance_read {
                    *quote_lots_available += get_approved_quote_lots(context, trader);

                    *quote_allowance_read = true;
                }

                return *quote_lots_available >= quote_lots_required;
            }
        }
    }
    true
}
