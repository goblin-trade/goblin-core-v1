use stylus_sdk::alloy_primitives::{Address, B256};

use crate::{
    parameters::{BASE_LOTS_PER_BASE_UNIT, TICK_SIZE_IN_QUOTE_LOTS_PER_BASE_UNIT},
    program::{FailedToReduce, GoblinError, GoblinResult, ReduceOrderPacket},
    quantities::{BaseLots, QuoteLots},
};

use super::{
    BitmapGroup, IndexList, MarketState, MatchingEngineResponse, OrderId, OuterIndex, Side,
    SlotActions, SlotRestingOrder, SlotStorage, TickIndices, TraderState,
};
use alloc::vec::Vec;

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

    /// Try to reduce a resting order.
    ///
    /// Returns None if the order doesn't exist or belongs to another trader.
    ///
    /// # State updations
    ///
    /// - order: Read, updated and stored
    /// - trader_state: Only updated
    ///
    /// # Arguments
    ///
    /// * `trader_state`
    /// * `trader`
    /// * `order_id`
    /// * `side`
    /// * `lots_to_remove` - Try to reduce size by this many lots. Pass u64::MAX to close entire order
    /// * `order_is_expired`
    /// * `claim_funds`
    ///
    pub fn reduce_order_inner(
        &mut self,
        trader_state: &mut TraderState,
        trader: Address,
        order_id: &OrderId,
        side: Side,
        lots_to_remove: BaseLots,
        order_is_expired: bool,
        claim_funds: bool,
    ) -> Option<ReduceOrderInnerResponse> {
        let mut order = SlotRestingOrder::new_from_slot(self.slot_storage, order_id);

        // Find lots to remove
        let (should_remove_order_from_book, base_lots_to_remove) = {
            // Order does not exist, or belongs to another trader
            if order.trader_address != trader {
                return None;
            }

            // If the order is tagged as expired, we remove it from the book regardless of the size.
            if order_is_expired {
                (true, order.num_base_lots)
            } else {
                let base_lots_to_remove = order.num_base_lots.min(lots_to_remove);

                (
                    base_lots_to_remove == order.num_base_lots,
                    base_lots_to_remove,
                )
            }
        };

        // Mutate order
        let _base_lots_remaining = if should_remove_order_from_book {
            order.clear_order();

            BaseLots::ZERO
        } else {
            // Reduce order
            order.num_base_lots -= base_lots_to_remove;

            order.num_base_lots
        };

        // EMIT ExpiredOrder / Reduce

        // Store order state
        order.write_to_slot(self.slot_storage, order_id).ok();

        // We don't want to claim funds if an order is removed from the book during a self trade
        // or if the user specifically indicates that they don't want to claim funds.
        if claim_funds {
            // Update trader state
            let (num_quote_lots, num_base_lots) = {
                match side {
                    Side::Bid => {
                        let quote_lots = (order_id.price_in_ticks
                            * TICK_SIZE_IN_QUOTE_LOTS_PER_BASE_UNIT
                            * base_lots_to_remove)
                            / BASE_LOTS_PER_BASE_UNIT;
                        trader_state.unlock_quote_lots(quote_lots);

                        (quote_lots, BaseLots::ZERO)
                    }
                    Side::Ask => {
                        trader_state.unlock_base_lots(base_lots_to_remove);

                        (QuoteLots::ZERO, base_lots_to_remove)
                    }
                }
            };

            Some(ReduceOrderInnerResponse {
                matching_engine_response: trader_state
                    .claim_funds_inner(num_quote_lots, num_base_lots),
                should_remove_order_from_book,
            })
        } else {
            // No claim case- the order is reduced but no funds will be claimed
            Some(ReduceOrderInnerResponse {
                matching_engine_response: MatchingEngineResponse::default(),
                should_remove_order_from_book,
            })
        }
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
        trader: Address,
        order_packets: Vec<B256>,
        claim_funds: bool,
    ) -> GoblinResult<MatchingEngineResponse> {
        // Read states
        let mut market = MarketState::read_from_slot(self.slot_storage);
        let mut trader_state = TraderState::read_from_slot(self.slot_storage, trader);

        let mut quote_lots_released = QuoteLots::ZERO;
        let mut base_lots_released = BaseLots::ZERO;

        let mut cached_bitmap_group: Option<(BitmapGroup, OuterIndex)> = None;

        let mut bid_index_list = IndexList::new(Side::Bid, market.bids_outer_indices);
        let mut ask_index_list = IndexList::new(Side::Ask, market.asks_outer_indices);

        for order_packet_bytes in order_packets {
            let ReduceOrderPacket {
                order_id,
                lots_to_remove: size,
                revert_if_fail,
            } = ReduceOrderPacket::decode(&order_packet_bytes);

            let side = order_id.side(market.best_bid_price, market.best_ask_price);

            if let Some(ReduceOrderInnerResponse {
                matching_engine_response,
                should_remove_order_from_book,
            }) = self.reduce_order_inner(
                &mut trader_state,
                trader,
                &order_id,
                side.clone(),
                size,
                false,
                claim_funds,
            ) {
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
                            BitmapGroup::new_from_slot(self.slot_storage, &outer_index),
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

        market.update_best_price(&bid_index_list, self.slot_storage);
        market.update_best_price(&ask_index_list, self.slot_storage);

        // write market state, trader state
        market.write_to_slot(self.slot_storage)?;
        trader_state.write_to_slot(self.slot_storage, trader);

        Ok(MatchingEngineResponse::new_withdraw(
            base_lots_released,
            quote_lots_released,
        ))
    }
}

pub struct ReduceOrderInnerResponse {
    pub matching_engine_response: MatchingEngineResponse,
    pub should_remove_order_from_book: bool,
}
