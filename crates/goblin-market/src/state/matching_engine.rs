use stylus_sdk::{
    alloy_primitives::{Address, FixedBytes, B256},
    block,
};

use crate::{
    parameters::{
        BASE_LOTS_PER_BASE_UNIT, BASE_LOT_SIZE, TAKER_FEE_BPS,
        TICK_SIZE_IN_QUOTE_LOTS_PER_BASE_UNIT,
    },
    program::{
        get_available_base_lots, get_available_quote_lots,
        new_order::{CondensedOrder, FailedMultipleLimitOrderBehavior},
        FailedToReduce, GoblinError, GoblinResult, OrderToInsert, PricesNotInOrder,
        ReduceOrderPacket,
    },
    quantities::{
        AdjustedQuoteLots, BaseLots, BaseLotsPerBaseUnit, QuoteLots, QuoteLotsPerBaseUnit, Ticks,
        WrapperU64, MAX_TICK,
    },
    require, GoblinMarket,
};

use super::{
    adjusted_quote_lot_budget_post_fee_adjustment_for_buys,
    adjusted_quote_lot_budget_post_fee_adjustment_for_sells, compute_fee, inner_indices,
    matching_engine_response, process_resting_orders, slot_storage, BitmapGroup, IndexList,
    InflightOrder, InnerIndex, ListKey, ListSlot, MarketState, MatchingEngineResponse,
    MutableBitmap, OrderId, OrderPacket, OrderPacketMetadata, OuterIndex, RestingOrder,
    RestingOrderIndex, Side, SlotActions, SlotRestingOrder, SlotStorage, TickIndices, TraderState,
};
use alloc::vec;
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

    /// Try to execute an order packet and place an order
    ///
    /// # Arguments
    ///
    /// * `market_state`
    /// * `trader_state`
    /// * `trader` - The trader who sent the order
    /// * `order_packet`
    /// * `last_order` - The last placed order, if placing multiple post-only orders
    ///
    pub fn place_order_inner(
        &mut self,
        market_state: &mut MarketState,
        trader_state: &mut TraderState,
        trader: Address,
        order_packet: &mut OrderPacket,
        last_order: Option<OrderToInsert>,
    ) -> Option<(Option<OrderToInsert>, MatchingEngineResponse)> {
        let side = order_packet.side();

        // Validate and try to correct tick price
        match side {
            Side::Bid => {
                if order_packet.get_price_in_ticks() == Ticks::ZERO {
                    // Bid price is too low
                    return None;
                }
            }
            Side::Ask => {
                if !order_packet.is_take_only() {
                    let tick_price = order_packet.get_price_in_ticks();
                    // Price cannot be zero. Set to 1
                    order_packet.set_price_in_ticks(tick_price.max(Ticks::ONE));
                }
            }
        }

        // Validate lots
        if order_packet.num_base_lots() == 0 && order_packet.num_quote_lots() == 0 {
            // Either num_base_lots or num_quote_lots must be nonzero
            return None;
        }

        // Validate IOC case
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

        // Fail if order packet expired
        if order_packet.is_expired(current_block, current_unix_timestamp) {
            // Do not fail the transaction if the order is expired, but do not place or match the order
            return Some((None, MatchingEngineResponse::default()));
        }

        // Generate resting_order and matching_engine_response
        let (resting_order, mut matching_engine_response) = if let OrderPacket::PostOnly {
            price_in_ticks,
            reject_post_only,
            ..
        } = order_packet
        {
            // If the current order crosses too, set the price equal to the last price
            if let Some(last_order) = last_order {
                let last_price = last_order.order_id.price_in_ticks;

                if (side == Side::Bid && *price_in_ticks > last_price)
                    || (side == Side::Ask && *price_in_ticks < last_price)
                {
                    *price_in_ticks = last_price;
                }
            } else if let Some(ticks) = self.check_for_cross(
                market_state,
                side,
                *price_in_ticks,
                current_block,
                current_unix_timestamp,
            ) {
                if *reject_post_only {
                    // PostOnly order crosses the book- order rejected
                    return None;
                } else {
                    // Try to amend order so it does not cross
                    match side {
                        Side::Bid => {
                            if ticks <= Ticks::ONE {
                                // PostOnly order crosses the book and can not be amended to a valid price- order rejected
                                return None;
                            }
                            *price_in_ticks = ticks - Ticks::ONE;
                        }
                        Side::Ask => {
                            // The MAX tick can never cross. No need to have ticks == Ticks::MAX case
                            *price_in_ticks = ticks + Ticks::ONE;
                        }
                    }
                }
            }

            (
                SlotRestingOrder::new(
                    trader,
                    order_packet.num_base_lots(),
                    order_packet.track_block(),
                    order_packet.last_valid_block_or_unix_timestamp_in_seconds(),
                ),
                MatchingEngineResponse::default(),
            )
        } else {
            // Limit and IOC order types

            let base_lot_budget = order_packet.base_lot_budget();
            // Multiply the quote lot budget by the number of base lots per unit to get the number of
            // adjusted quote lots (quote_lots * base_lots_per_base_unit)
            let quote_lot_budget = order_packet.quote_lot_budget();

            let adjusted_quote_lot_budget = match side {
                // For buys, the adjusted quote lot budget is decreased by the max fee.
                // This is because the fee is added to the quote lots spent after the matching is complete.
                Side::Bid => quote_lot_budget.and_then(|quote_lot_budget| {
                    adjusted_quote_lot_budget_post_fee_adjustment_for_buys(
                        quote_lot_budget * BASE_LOTS_PER_BASE_UNIT,
                    )
                }),
                // For sells, the adjusted quote lot budget is increased by the max fee.
                // This is because the fee is subtracted from the quote lot received after the matching is complete.
                Side::Ask => quote_lot_budget.and_then(|quote_lot_budget| {
                    adjusted_quote_lot_budget_post_fee_adjustment_for_sells(
                        quote_lot_budget * BASE_LOTS_PER_BASE_UNIT,
                    )
                }),
            }
            .unwrap_or_else(|| AdjustedQuoteLots::new(u64::MAX));

            let mut inflight_order = InflightOrder::new(
                side,
                order_packet.self_trade_behavior(),
                order_packet.get_price_in_ticks(),
                order_packet.match_limit(),
                base_lot_budget,
                adjusted_quote_lot_budget,
                order_packet.track_block(),
                order_packet.last_valid_block_or_unix_timestamp_in_seconds(),
            );

            // Gives the number of unmatched base lots
            // For limit orders- place a new resting order with this amount
            // For IOC- test against threshold
            let resting_order = self
                .match_order(
                    market_state,
                    &mut inflight_order,
                    trader,
                    current_block,
                    current_unix_timestamp,
                )
                .map_or_else(|| None, Some)?;

            // matched_adjusted_quote_lots is rounded down to the nearest tick for buys and up for
            // sells to yield a whole number of matched_quote_lots.
            let matched_quote_lots = match side {
                // We add the quote_lot_fees to account for the fee being paid on a buy order
                Side::Bid => {
                    (round_adjusted_quote_lots_up(inflight_order.matched_adjusted_quote_lots)
                        / BASE_LOTS_PER_BASE_UNIT)
                        + inflight_order.quote_lot_fees
                }
                // We subtract the quote_lot_fees to account for the fee being paid on a sell order
                Side::Ask => {
                    (round_adjusted_quote_lots_down(inflight_order.matched_adjusted_quote_lots)
                        / BASE_LOTS_PER_BASE_UNIT)
                        - inflight_order.quote_lot_fees
                }
            };
            let matching_engine_response = match side {
                Side::Bid => MatchingEngineResponse::new_from_buy(
                    matched_quote_lots,
                    inflight_order.matched_base_lots,
                ),
                Side::Ask => MatchingEngineResponse::new_from_sell(
                    inflight_order.matched_base_lots,
                    matched_quote_lots,
                ),
            };

            // EMIT FillSummary

            (resting_order, matching_engine_response)
        };

        let mut order_to_insert: Option<OrderToInsert> = None;

        if let OrderPacket::ImmediateOrCancel {
            min_base_lots_to_fill,
            min_quote_lots_to_fill,
            ..
        } = order_packet
        {
            // For IOC orders, if the order's minimum fill requirements are not met, then
            // the order is voided
            if matching_engine_response.num_base_lots() < *min_base_lots_to_fill
                || matching_engine_response.num_quote_lots() < *min_quote_lots_to_fill
            {
                // IOC order failed to meet minimum fill requirements.
                return None;
            }
        } else {
            // PostOnly and limit case- place an order on the book
            // Get best available slot to place the order
            let best_available_order_id =
                self.get_best_available_order_id(&order_packet, last_order);

            match best_available_order_id {
                None => {
                    // No space for order, exit
                    // Multiple orders behavior is handled outside
                    // Currently the entire TX fails
                    return None;
                }
                Some(order_id) => {
                    if resting_order.num_base_lots > BaseLots::ZERO {
                        // Queue resting order for insertion, update states and matching engine response
                        // This happens only in limit and post-only case, not IOC
                        order_to_insert = Some(OrderToInsert {
                            order_id,
                            resting_order,
                        });

                        // Update trader state and matching engine response
                        match side {
                            Side::Bid => {
                                let quote_lots_to_lock = (TICK_SIZE_IN_QUOTE_LOTS_PER_BASE_UNIT
                                    * order_id.price_in_ticks
                                    * resting_order.num_base_lots)
                                    / BASE_LOTS_PER_BASE_UNIT;
                                let quote_lots_free_to_use =
                                    quote_lots_to_lock.min(trader_state.quote_lots_free);
                                trader_state.use_free_quote_lots(quote_lots_free_to_use);
                                trader_state.lock_quote_lots(quote_lots_to_lock);
                                matching_engine_response.post_quote_lots(quote_lots_to_lock);
                                matching_engine_response
                                    .use_free_quote_lots(quote_lots_free_to_use);
                            }
                            Side::Ask => {
                                let base_lots_free_to_use =
                                    resting_order.num_base_lots.min(trader_state.base_lots_free);
                                trader_state.use_free_base_lots(base_lots_free_to_use);
                                trader_state.lock_base_lots(resting_order.num_base_lots);
                                matching_engine_response
                                    .post_base_lots(resting_order.num_base_lots);
                                matching_engine_response.use_free_base_lots(base_lots_free_to_use);
                            }
                        }

                        // EMIT Place
                        // EMIT TimeInForce if this is a time in force order
                    }
                }
            }
        }

        // Limit and post-only branch ends
        // Update the trader state and matching engine response

        // Check if trader has free lots
        match side {
            Side::Bid => {
                let quote_lots_free_to_use = trader_state
                    .quote_lots_free
                    .min(matching_engine_response.num_quote_lots());
                trader_state.use_free_quote_lots(quote_lots_free_to_use);
                matching_engine_response.use_free_quote_lots(quote_lots_free_to_use);
            }
            Side::Ask => {
                let base_lots_free_to_use = trader_state
                    .base_lots_free
                    .min(matching_engine_response.num_base_lots());
                trader_state.use_free_base_lots(base_lots_free_to_use);
                matching_engine_response.use_free_base_lots(base_lots_free_to_use);
            }
        }

        // If the order crosses and only uses deposited funds, then add the matched funds back to the trader's free funds
        // Set the matching_engine_response lots_out to zero to set token withdrawals to zero
        if order_packet.no_deposit_or_withdrawal() {
            match side {
                Side::Bid => {
                    trader_state.deposit_free_base_lots(matching_engine_response.num_base_lots_out);
                    matching_engine_response.num_base_lots_out = BaseLots::ZERO;
                }
                Side::Ask => {
                    trader_state
                        .deposit_free_quote_lots(matching_engine_response.num_quote_lots_out);
                    matching_engine_response.num_quote_lots_out = QuoteLots::ZERO;
                }
            }

            // Check if trader has enough deposited funds to process the order
            // and no tokens are withdrawn
            if !matching_engine_response.verify_no_deposit()
                || !matching_engine_response.verify_no_withdrawal()
            {
                return None;
            }
        }

        Some((order_to_insert, matching_engine_response))
    }

    /// Insert multiple resting orders in the book
    /// This is used to write multiple post-only orders.
    ///
    /// # Arguments
    ///
    /// * `market_state`
    /// * `side`
    /// * `orders`
    ///
    pub fn insert_orders_in_book_bulk(
        &mut self,
        market_state: &mut MarketState,
        side: Side,
        orders: Vec<OrderToInsert>,
    ) -> GoblinResult<()> {
        // 1. Update market state
        // Optimization- since the first element is closest to the centre, we only need
        // to check the first element against the current best price.
        // Update the best price if necessary.
        if let Some(first_order) = orders.get(0) {
            let order_id = first_order.order_id;

            // Update best market price
            if side == Side::Bid && order_id.price_in_ticks > market_state.best_bid_price {
                market_state.best_bid_price = order_id.price_in_ticks;
            }

            if side == Side::Ask && order_id.price_in_ticks < market_state.best_ask_price {
                market_state.best_ask_price = order_id.price_in_ticks;
            }
        } else {
            // No orders
            return Ok(());
        }

        let mut last_outer_index = None;
        let mut bitmap_group = BitmapGroup::default();

        // Process orders and handle bitmap updates
        for (
            OrderToInsert {
                order_id,
                resting_order,
            },
            inserted,
        ) in orders.iter().zip(
            // 2. Insert into index list
            self.insert_to_index_list_bulk(
                market_state,
                side,
                orders
                    .iter()
                    .map(|o| o.order_id.price_in_ticks.outer_index())
                    .collect(),
            ),
        ) {
            // 3. Write resting order to slot
            resting_order.write_to_slot(self.slot_storage, &order_id)?;

            let TickIndices {
                outer_index,
                inner_index,
            } = order_id.price_in_ticks.to_indices();

            if last_outer_index != Some(outer_index) {
                // New index encountered. Write bitmap group belonging to the old index to slot.
                if let Some(last_index) = last_outer_index {
                    bitmap_group.write_to_slot(self.slot_storage, &last_index);
                }

                bitmap_group = if inserted {
                    BitmapGroup::default()
                } else {
                    BitmapGroup::new_from_slot(self.slot_storage, outer_index)
                };

                last_outer_index = Some(outer_index);
            }

            // 4. Flip tick in bitmap
            let mut bitmap = bitmap_group.get_bitmap_mut(&inner_index);
            bitmap.activate(&order_id.resting_order_index);
        }

        // Write the final bitmap group to the slot storage
        if let Some(outer_index) = last_outer_index {
            bitmap_group.write_to_slot(self.slot_storage, &outer_index);
        }

        Ok(())
    }

    pub fn insert_order_in_book(
        &mut self,
        market_state: &mut MarketState,
        resting_order: &SlotRestingOrder,
        side: Side,
        order_id: &OrderId,
    ) -> GoblinResult<()> {
        // Write resting order to slot
        resting_order.write_to_slot(self.slot_storage, order_id)?;

        // Update best market price
        if side == Side::Bid && order_id.price_in_ticks > market_state.best_bid_price {
            market_state.best_bid_price = order_id.price_in_ticks;
        }

        if side == Side::Ask && order_id.price_in_ticks < market_state.best_ask_price {
            market_state.best_ask_price = order_id.price_in_ticks;
        }

        // Try to insert outer index in index list
        let TickIndices {
            outer_index,
            inner_index,
        } = order_id.price_in_ticks.to_indices();
        let inserted = self.insert_to_index_list(market_state, outer_index, side);

        let mut bitmap_group = if inserted {
            // The bitmap group could be in 'cleared' state with placeholder bits. We need
            // to start with empty bits
            BitmapGroup::default()
        } else {
            BitmapGroup::new_from_slot(self.slot_storage, outer_index)
        };

        let mut bitmap = bitmap_group.get_bitmap_mut(&inner_index);
        bitmap.activate(&order_id.resting_order_index);
        bitmap_group.write_to_slot(self.slot_storage, &outer_index);

        Ok(())
    }

    /// Insert a vector of outer indices in the outer indices list
    ///
    /// Returns a bool array telling which index was inserted (true case), and which one was
    /// already present in the list (false case)
    ///
    /// # Arguments
    ///
    /// * `market_state`
    /// * `side`
    /// * `input_list`
    ///
    pub fn insert_to_index_list_bulk(
        &mut self,
        market_state: &MarketState,
        side: Side,
        input_list: Vec<OuterIndex>,
    ) -> Vec<bool> {
        let input_list_size = input_list.len();

        let mut result = vec![false; input_list_size];

        if input_list_size == 0 {
            return result;
        }
        let mut input_list_index = 0;

        let outer_index_count = market_state.outer_index_length(side);

        let initial_slot_index = (outer_index_count - 1) / 16;
        let mut slot_index = initial_slot_index;
        let mut relative_index = (outer_index_count - 1) % 16;

        let mut outer_index_stack = Vec::<OuterIndex>::new();
        let mut to_insert = false;

        let mut list_slot =
            ListSlot::new_from_slot(self.slot_storage, ListKey { index: slot_index });

        // 1. Loop through index slots to generate outer_index_stack and find indices
        'list_loop: loop {
            // List slot for the first slot index is already loaded, don't read again
            if slot_index != initial_slot_index {
                let list_key = ListKey { index: slot_index };
                list_slot = ListSlot::new_from_slot(self.slot_storage, list_key);
            }

            // 2. Loop through outer indices in the list slot
            loop {
                // 3. Loop through values_to_insert to get the next non-duplicate outer index
                let mut outer_index = input_list.get(input_list_index).unwrap();
                loop {
                    let last_stacked_outer_index = outer_index_stack.last();

                    if last_stacked_outer_index.is_some_and(|last_stacked_outer_index| {
                        last_stacked_outer_index == outer_index
                    }) {
                        if input_list_index == input_list_size - 1 {
                            break 'list_loop;
                        }
                        input_list_index += 1;
                        outer_index = input_list.get(input_list_index).unwrap();
                    } else {
                        break;
                    }
                }

                let current_outer_index = list_slot.get(relative_index as usize);

                // Case 1- need to travel deeper in index list
                if (side == Side::Bid && current_outer_index > *outer_index)
                    || (side == Side::Ask && current_outer_index < *outer_index)
                {
                    // Need to traverse further
                    // Stack the item read from the index list
                    outer_index_stack.push(current_outer_index);
                } else {
                    // Case 2- insert the index at this position
                    if current_outer_index != *outer_index {
                        outer_index_stack.push(*outer_index);
                        result[input_list_index] = true;
                        to_insert = true;
                    }

                    // General case- either insert or the value is already present
                    // Try to move to the next item in the input list
                    if input_list_index == input_list_size - 1 {
                        break 'list_loop;
                    }
                    input_list_index += 1;
                }
                if relative_index == 0 {
                    break;
                }
                relative_index -= 1;
            }

            if slot_index == 0 {
                break;
            }
            // Move to the next ListSlot. Reset the relative index.
            slot_index -= 1;
            relative_index = 15;
        }

        if !to_insert {
            return result;
        }

        // 2. Write the stack to the list
        let new_list_size = slot_index * 16 + relative_index + 1 + outer_index_stack.len() as u16;

        // Start from one position ahead of the last read item
        let starting_slot_index = slot_index + (relative_index + 1) / 16;
        let mut starting_relative_index = (relative_index + 1) % 16;

        let final_slot_index = (new_list_size - 1) / 16;

        for slot_index in starting_slot_index..=final_slot_index {
            // Avoid duplicate slot load for starting_slot_index
            if slot_index != starting_slot_index {
                list_slot = ListSlot::default();
                starting_relative_index = 0;
            }

            let final_relative_index = if slot_index == final_slot_index {
                (new_list_size - 1) % 16
            } else {
                16
            };

            for relative_index in starting_relative_index..=final_relative_index {
                list_slot.set(relative_index as usize, outer_index_stack.pop().unwrap());
            }
            list_slot.write_to_slot(
                self.slot_storage,
                &ListKey {
                    index: relative_index,
                },
            );
        }

        result
    }

    // Try to insert an outer index in the index list.
    // Returns true if insertion was necessary, returns false if the outer index
    // is already present.
    //
    // TODO move to index_list.rs and cleanup that file
    //
    // # Arguments
    //
    // * `market_state`
    // * `outer_index` - The value to insert
    // * `side`
    //
    fn insert_to_index_list(
        &mut self,
        market_state: &MarketState,
        outer_index: OuterIndex,
        side: Side,
    ) -> bool {
        let outer_index_count = market_state.outer_index_length(side);

        let initial_slot_index = (outer_index_count - 1) / 16;
        let mut slot_index = initial_slot_index;
        let mut relative_index = (outer_index_count - 1) % 16;

        let mut outer_index_stack = Vec::<OuterIndex>::new();
        let mut to_insert = false;

        let mut list_slot =
            ListSlot::new_from_slot(self.slot_storage, ListKey { index: slot_index });

        // 1. Loop through index slots to generate outer_index_stack and find indices
        // slot_index in (0..=initial_slot_index).rev()
        'list_loop: loop {
            // List slot for the first slot index is already loaded, don't read again
            if slot_index != initial_slot_index {
                let list_key = ListKey { index: slot_index };
                list_slot = ListSlot::new_from_slot(self.slot_storage, list_key);
            }

            // 2. Loop through outer indices in the list slot
            loop {
                let current_outer_index = list_slot.get(relative_index as usize);

                if current_outer_index == outer_index {
                    // Outer index is already present, no need to insert. Exit from loop,
                    // to_insert remains false
                    break 'list_loop;
                } else if (side == Side::Bid && current_outer_index > outer_index)
                    || (side == Side::Ask && current_outer_index < outer_index)
                {
                    outer_index_stack.push(current_outer_index);
                } else {
                    // Insert at this index
                    to_insert = true;
                    break 'list_loop;
                }

                if relative_index == 0 {
                    break;
                }
                relative_index -= 1;
            }

            if slot_index == 0 {
                break;
            }
            // Move to the next ListSlot. Reset the relative index.
            slot_index -= 1;
            relative_index = 15;
        }

        if !to_insert {
            return false;
        }

        // 2. Add the outer index to add to the stack
        outer_index_stack.push(outer_index);

        // 3. Write the stack to the list
        let new_list_size = outer_index_count + 1;

        // if new_list_size = 1 then final_slot_index_exclusive will become 0.
        // But we want to iterate across slot 0.
        let final_slot_index = (new_list_size - 1) / 16;

        // Start from one position ahead of the last read item
        let mut starting_relative_index = (relative_index + 1) % 16;
        let starting_slot_index = slot_index + (relative_index + 1) / 16;

        for slot_index in starting_slot_index..=final_slot_index {
            // Avoid duplicate slot load for starting_slot_index
            if slot_index != starting_slot_index {
                list_slot = ListSlot::default();
                starting_relative_index = 0;
            }

            let final_relative_index = if slot_index == final_slot_index {
                (new_list_size - 1) % 16
            } else {
                16
            };

            for relative_index in starting_relative_index..=final_relative_index {
                list_slot.set(relative_index as usize, outer_index_stack.pop().unwrap());
            }
            list_slot.write_to_slot(
                self.slot_storage,
                &ListKey {
                    index: relative_index,
                },
            );
        }
        true
    }

    /// Match the inflight order with crossing resting orders of the opposite side.
    ///
    /// Returns a SlotRestingOrder which for
    /// - Limit case: should be posted as a resting order
    /// - IOC case: is used to validate fill conditions
    ///
    fn match_order(
        &mut self,
        market_state: &mut MarketState,
        inflight_order: &mut InflightOrder,
        taker_address: Address,
        current_block: u32,
        current_unix_timestamp_in_seconds: u32,
    ) -> Option<SlotRestingOrder> {
        let mut abort = false;
        let mut total_matched_adjusted_quote_lots = AdjustedQuoteLots::ZERO;
        let opposite_side = inflight_order.side.opposite();

        let mut handle_match = |order_id: OrderId,
                                resting_order: &mut SlotRestingOrder,
                                slot_storage: &mut SlotStorage| {
            let num_base_lots_quoted = resting_order.num_base_lots;

            let crosses = match inflight_order.side.opposite() {
                Side::Bid => order_id.price_in_ticks >= inflight_order.limit_price_in_ticks,
                Side::Ask => order_id.price_in_ticks <= inflight_order.limit_price_in_ticks,
            };

            if !crosses {
                return true;
            }

            let mut maker_state =
                TraderState::read_from_slot(slot_storage, resting_order.trader_address);

            // 1. Resting order expired case
            if resting_order.expired(current_block, current_unix_timestamp_in_seconds) {
                resting_order
                    .reduce_order(
                        &mut maker_state,
                        resting_order.trader_address,
                        &order_id,
                        inflight_order.side.opposite(),
                        BaseLots::MAX,
                        true,
                        false,
                    )
                    .unwrap();
                maker_state.write_to_slot(slot_storage, resting_order.trader_address);
                inflight_order.match_limit -= 1;

                // If match limit is exhausted, this returns false to stop
                return !inflight_order.in_progress();
            }

            // 2. Self trade case
            if taker_address == resting_order.trader_address {
                match inflight_order.self_trade_behavior {
                    crate::state::SelfTradeBehavior::Abort => {
                        abort = true;
                        return true;
                    }
                    crate::state::SelfTradeBehavior::CancelProvide => {
                        // Cancel the resting order without charging fees.

                        resting_order
                            .reduce_order(
                                &mut maker_state,
                                taker_address,
                                &order_id,
                                inflight_order.side.opposite(),
                                BaseLots::MAX,
                                false,
                                false,
                            )
                            .unwrap();
                        maker_state.write_to_slot(slot_storage, resting_order.trader_address);

                        inflight_order.match_limit -= 1;
                    }
                    crate::state::SelfTradeBehavior::DecrementTake => {
                        // Match against the maker order, but don't add fees
                        // Similar matching logic is used later, but here the amount matched is
                        // not added to total_matched_adjusted_quote_lots
                        let base_lots_removed = inflight_order
                            .base_lot_budget
                            .min(
                                inflight_order
                                    .adjusted_quote_lot_budget
                                    .unchecked_div::<QuoteLotsPerBaseUnit, BaseLots>(
                                        order_id.price_in_ticks
                                            * TICK_SIZE_IN_QUOTE_LOTS_PER_BASE_UNIT,
                                    ),
                            )
                            .min(num_base_lots_quoted);

                        resting_order
                            .reduce_order(
                                &mut maker_state,
                                taker_address,
                                &order_id,
                                inflight_order.side.opposite(),
                                base_lots_removed,
                                false,
                                false,
                            )
                            .unwrap();

                        maker_state.write_to_slot(slot_storage, resting_order.trader_address);

                        // In the case that the self trade behavior is DecrementTake, we decrement the
                        // the base lot and adjusted quote lot budgets accordingly
                        inflight_order.base_lot_budget = inflight_order
                            .base_lot_budget
                            .saturating_sub(base_lots_removed);
                        inflight_order.adjusted_quote_lot_budget =
                            inflight_order.adjusted_quote_lot_budget.saturating_sub(
                                TICK_SIZE_IN_QUOTE_LOTS_PER_BASE_UNIT
                                    * order_id.price_in_ticks
                                    * base_lots_removed,
                            );
                        // Self trades will count towards the match limit
                        inflight_order.match_limit -= 1;
                    }
                }
                return !inflight_order.in_progress();
            }

            let num_adjusted_quote_lots_quoted = order_id.price_in_ticks
                * TICK_SIZE_IN_QUOTE_LOTS_PER_BASE_UNIT
                * num_base_lots_quoted;

            // Use matched_base_lots and matched_adjusted_quote_lots to update the
            // inflight order and trader state
            let (matched_base_lots, matched_adjusted_quote_lots) = {
                // Check if the inflight order's budget is exhausted
                // Compare inflight order's budgets with quoted lots
                let has_remaining_adjusted_quote_lots =
                    num_adjusted_quote_lots_quoted <= inflight_order.adjusted_quote_lot_budget;
                let has_remaining_base_lots =
                    num_base_lots_quoted <= inflight_order.base_lot_budget;

                // Budget exceeds quote. Clear the resting order.
                if has_remaining_base_lots && has_remaining_adjusted_quote_lots {
                    resting_order.clear_order();
                    (num_base_lots_quoted, num_adjusted_quote_lots_quoted)
                } else {
                    // If the order's budget is exhausted, we match as much as we can
                    let base_lots_to_remove = inflight_order.base_lot_budget.min(
                        inflight_order
                            .adjusted_quote_lot_budget
                            .unchecked_div::<QuoteLotsPerBaseUnit, BaseLots>(
                                order_id.price_in_ticks * TICK_SIZE_IN_QUOTE_LOTS_PER_BASE_UNIT,
                            ),
                    );
                    let adjusted_quote_lots_to_remove = order_id.price_in_ticks
                        * TICK_SIZE_IN_QUOTE_LOTS_PER_BASE_UNIT
                        * base_lots_to_remove;

                    resting_order.num_base_lots -= base_lots_to_remove;

                    (base_lots_to_remove, adjusted_quote_lots_to_remove)
                }
            };

            // Deplete the inflight order's budget by the amount matched
            inflight_order.process_match(matched_adjusted_quote_lots, matched_base_lots);

            // Increment the matched adjusted quote lots for fee calculation
            total_matched_adjusted_quote_lots += matched_adjusted_quote_lots;

            // EMIT fill event

            // Update the maker's state to reflect the match
            match inflight_order.side {
                Side::Bid => maker_state.process_limit_sell(
                    matched_base_lots,
                    matched_adjusted_quote_lots / BASE_LOTS_PER_BASE_UNIT,
                ),
                Side::Ask => maker_state.process_limit_buy(
                    matched_adjusted_quote_lots / BASE_LOTS_PER_BASE_UNIT,
                    matched_base_lots,
                ),
            }

            !inflight_order.in_progress()
        };

        process_resting_orders(
            self.slot_storage,
            market_state,
            opposite_side,
            &mut handle_match,
        );

        if abort {
            return None;
        }

        // Fees are updated based on the total amount matched

        inflight_order.quote_lot_fees =
            round_adjusted_quote_lots_up(compute_fee(total_matched_adjusted_quote_lots))
                / BASE_LOTS_PER_BASE_UNIT;

        market_state.unclaimed_quote_lot_fees += inflight_order.quote_lot_fees;

        Some(SlotRestingOrder {
            trader_address: taker_address,
            num_base_lots: inflight_order.base_lot_budget,
            track_block: inflight_order.track_block,
            last_valid_block_or_unix_timestamp_in_seconds: inflight_order
                .last_valid_block_or_unix_timestamp_in_seconds,
        })
    }

    /// Find the best available free order ID where a resting order can be placed,
    /// at `price` or better (away from centre).
    /// Returns None if no space is available for the given number of amendments.
    ///
    /// # Arguments
    ///
    /// * `order_packet`
    /// * `last_order` - The last order, if placing multiple post-only orders. If order id
    /// and expiry params match, then return the same order id as the last order.
    ///
    pub fn get_best_available_order_id(
        &mut self,
        order_packet: &OrderPacket,
        last_order: Option<OrderToInsert>,
    ) -> Option<OrderId> {
        let price_in_ticks = order_packet.get_price_in_ticks();
        let side = order_packet.side();

        // If the current and last order have the same order ID but different expiry
        // params, then construct a virtual bitmap where bit for the previous order is turned on.
        let mut skip_bit_for_last_order = false;

        if let Some(OrderToInsert {
            order_id,
            resting_order,
        }) = last_order
        {
            if order_id.price_in_ticks == price_in_ticks {
                // If expiry parameters are the same, then return same order id as
                // the previous order so that the two orders can be merged.
                if resting_order.track_block == order_packet.track_block()
                    && resting_order.last_valid_block_or_unix_timestamp_in_seconds
                        == order_packet.last_valid_block_or_unix_timestamp_in_seconds()
                {
                    return Some(order_id);
                } else {
                    skip_bit_for_last_order = true;
                }
            }
        }

        let TickIndices {
            outer_index,
            inner_index,
        } = price_in_ticks.to_indices();

        let mut current_outer_index = outer_index;
        let mut ticks_to_traverse = order_packet.tick_offset();

        // 1. Loop through bitmap groups
        loop {
            let bitmap_group = BitmapGroup::new_from_slot(self.slot_storage, current_outer_index);

            let previous_inner_index = if current_outer_index == outer_index {
                Some(inner_index)
            } else {
                None
            };

            // 2. Loop through bitmaps
            for i in inner_indices(side, previous_inner_index) {
                let current_inner_index = InnerIndex::new(i);
                let price_in_ticks = Ticks::from_indices(outer_index, current_inner_index);

                // 3. Loop through resting order IDs
                let best_free_index = if skip_bit_for_last_order {
                    // Mark as false. This is a one time operation for the first bitmap group
                    skip_bit_for_last_order = false;

                    // Construct a virtual bitmap which includes activated bit from the last order
                    let mut bitmap_raw = bitmap_group.inner[current_inner_index.as_usize()];
                    let mut virtual_bitmap = MutableBitmap {
                        inner: &mut bitmap_raw,
                    };
                    let relative_index_of_last_order =
                        last_order.unwrap().order_id.resting_order_index;
                    virtual_bitmap.activate(&relative_index_of_last_order);

                    // Lookup from relative_index_of_last_order. This index is filled so it
                    // will be skipped.
                    virtual_bitmap.best_free_index(relative_index_of_last_order.as_u8())
                } else {
                    let bitmap = bitmap_group.get_bitmap(&current_inner_index);

                    bitmap.best_free_index(0)
                };

                if let Some(resting_order_index) = best_free_index {
                    return Some(OrderId {
                        price_in_ticks,
                        resting_order_index,
                    });
                };

                if ticks_to_traverse == 0 {
                    return None;
                }
                ticks_to_traverse -= 1;
            }

            if side == Side::Bid {
                if current_outer_index == OuterIndex::ZERO {
                    break;
                }
                current_outer_index -= OuterIndex::ONE;
            } else {
                if current_outer_index == OuterIndex::MAX {
                    break;
                }
                current_outer_index += OuterIndex::ONE;
            }
        }

        None
    }

    /// This function determines whether a PostOnly order crosses the book.
    /// If the order crosses the book, the function returns the price of the best unexpired
    /// crossing order (price, index) on the opposite side of the book. Otherwise, it returns None.
    ///
    /// The function closes all expired orders till an unexpired order is found.
    ///
    /// # Arguments
    ///
    /// * `market_state`
    /// * `side`
    /// * `num_ticks`
    /// * `current_block`
    /// * `current_unix_timestamp_in_seconds`
    ///
    fn check_for_cross(
        &mut self,
        market_state: &mut MarketState,
        side: Side,
        limit_price_in_ticks: Ticks,
        current_block: u32,
        current_unix_timestamp_in_seconds: u32,
    ) -> Option<Ticks> {
        let opposite_side = side.opposite();
        let opposite_best_price = market_state.best_price(opposite_side);
        let outer_index_count = market_state.outer_index_length(opposite_side);

        if outer_index_count == 0 // Book empty case
            // No cross case
            || (side == Side::Bid && limit_price_in_ticks < opposite_best_price)
            || (side == Side::Ask && limit_price_in_ticks > opposite_best_price)
        {
            return None;
        }

        let mut crossing_tick: Option<Ticks> = None;

        let mut handle_cross = |order_id: OrderId,
                                resting_order: &mut SlotRestingOrder,
                                slot_storage: &mut SlotStorage| {
            let crosses = match side.opposite() {
                Side::Bid => order_id.price_in_ticks >= limit_price_in_ticks,
                Side::Ask => order_id.price_in_ticks <= limit_price_in_ticks,
            };

            if !crosses {
                return true;
            }

            if resting_order.expired(current_block, current_unix_timestamp_in_seconds) {
                let mut trader_state =
                    TraderState::read_from_slot(slot_storage, resting_order.trader_address);

                resting_order
                    .reduce_order(
                        &mut trader_state,
                        resting_order.trader_address,
                        &order_id,
                        side.opposite(),
                        BaseLots::MAX,
                        true,
                        false,
                    )
                    .unwrap();
                trader_state.write_to_slot(slot_storage, resting_order.trader_address);

                return false;
            }

            crossing_tick = Some(order_id.price_in_ticks);
            return true;
        };

        //
        process_resting_orders(
            self.slot_storage,
            market_state,
            opposite_side,
            &mut handle_cross,
        );

        crossing_tick
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
                    *base_lots_available += get_available_base_lots(context, trader);
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
                    *quote_lots_available += get_available_quote_lots(context, trader);

                    *quote_allowance_read = true;
                }

                return *quote_lots_available >= quote_lots_required;
            }
        }
    }
    true
}

/// Adjusted quote lots, rounded up to the nearest multiple of base_lots_per_base_unit
pub fn round_adjusted_quote_lots_up(
    num_adjusted_quote_lots: AdjustedQuoteLots,
) -> AdjustedQuoteLots {
    ((num_adjusted_quote_lots + AdjustedQuoteLots::new(BASE_LOTS_PER_BASE_UNIT.as_u64() - 1))
        .unchecked_div::<BaseLotsPerBaseUnit, QuoteLots>(BASE_LOTS_PER_BASE_UNIT))
        * BASE_LOTS_PER_BASE_UNIT
}

/// Adjusted quote lots, rounded down to the nearest multiple of base_lots_per_base_unit
pub fn round_adjusted_quote_lots_down(
    num_adjusted_quote_lots: AdjustedQuoteLots,
) -> AdjustedQuoteLots {
    num_adjusted_quote_lots.unchecked_div::<BaseLotsPerBaseUnit, QuoteLots>(BASE_LOTS_PER_BASE_UNIT)
        * BASE_LOTS_PER_BASE_UNIT
}
