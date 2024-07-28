use stylus_sdk::{
    alloy_primitives::{Address, B256},
    block,
};

use crate::{
    parameters::{BASE_LOTS_PER_BASE_UNIT, TAKER_FEE_BPS, TICK_SIZE_IN_QUOTE_LOTS_PER_BASE_UNIT},
    program::{
        get_approved_base_lots, get_approved_quote_lots,
        new_order::{CondensedOrder, FailedMultipleLimitOrderBehavior},
        FailedToReduce, GoblinError, GoblinResult, PricesNotInOrder, ReduceOrderPacket,
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
    InflightOrder, InnerIndex, ListKey, ListSlot, MarketState, MatchingEngineResponse, OrderId,
    OrderPacket, OrderPacketMetadata, OuterIndex, RestingOrder, RestingOrderIndex, Side,
    SlotActions, SlotRestingOrder, SlotStorage, TickIndices, TraderState,
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

    /// Try to execute an order packet
    ///
    /// # Arguments
    ///
    /// * `market_state`
    /// * `trader_state`
    /// * `trader` - The trader who sent the order
    /// * `to` - Credit the output to this address
    /// * `order_packet`
    ///
    fn place_order_inner(
        &mut self,
        market_state: &mut MarketState,
        trader_state: &mut TraderState,
        trader: Address,
        to: Address,
        order_packet: &mut OrderPacket,
    ) -> GoblinResult<Option<MatchingEngineResponse>> {
        let side = order_packet.side();

        match side {
            Side::Bid => {
                if order_packet.get_price_in_ticks() == Ticks::ZERO {
                    // Bid price is too low
                    return Ok(None);
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

        if order_packet.num_base_lots() == 0 && order_packet.num_quote_lots() == 0 {
            // Either num_base_lots or num_quote_lots must be nonzero
            return Ok(None);
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
                return Ok(None);
            }
        }

        let current_block = block::number() as u32;
        let current_unix_timestamp = block::timestamp() as u32;

        if order_packet.is_expired(current_block, current_unix_timestamp) {
            // Do not fail the transaction if the order is expired, but do not place or match the order
            return Ok(Some(MatchingEngineResponse::default()));
        }

        let mut index_list = market_state.get_index_list(side);

        // Build resting_order and matching_engine_response
        let (resting_order, mut matching_engine_response) = if let OrderPacket::PostOnly {
            price_in_ticks,
            reject_post_only,
            ..
        } = order_packet
        {
            if let Some(order_id) = self.check_for_cross(
                market_state,
                side,
                *price_in_ticks,
                current_block,
                current_unix_timestamp,
            ) {
                let ticks = order_id.price_in_ticks;

                if *reject_post_only {
                    // PostOnly order crosses the book- order rejected
                    return Ok(None);
                } else {
                    // Try to amend order so it does not cross
                    match side {
                        Side::Bid => {
                            if ticks <= Ticks::ONE {
                                // PostOnly order crosses the book and can not be amended to a valid price- order rejected
                                return Ok(None);
                            }
                            *price_in_ticks = ticks - Ticks::ONE;
                        }
                        Side::Ask => {
                            if ticks == Ticks::MAX {
                                // Prevent overflow
                                return Ok(None);
                            }
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

            (
                SlotRestingOrder::new_default(trader, BaseLots::ZERO),
                MatchingEngineResponse::default(),
            )
        };

        Ok(None)
    }

    /// Match the inflight orders with crossing resting orders of the opposite side
    fn match_order(
        &mut self,
        market_state: &mut MarketState,
        inflight_order: &mut InflightOrder,
        taker_address: Address,
        current_block: u32,
        current_unix_timestamp_in_seconds: u32,
    ) {
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

        // Fees are updated based on the total amount matched

        inflight_order.quote_lot_fees =
            round_adjusted_quote_lots_up(compute_fee(total_matched_adjusted_quote_lots))
                / BASE_LOTS_PER_BASE_UNIT;

        market_state.unclaimed_quote_lot_fees += inflight_order.quote_lot_fees;
    }

    /// This function determines whether a PostOnly order crosses the book.
    /// If the order crosses the book, the function returns the ID of the best unexpired
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
    ) -> Option<OrderId> {
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

        let mut crossing_tick: Option<OrderId> = None;

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

            crossing_tick = Some(order_id);
            return true;
        };

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
