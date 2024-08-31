use stylus_sdk::{
    alloy_primitives::{Address, B256},
    block,
};

use crate::{
    parameters::{BASE_LOTS_PER_BASE_UNIT, TICK_SIZE_IN_QUOTE_LOTS_PER_BASE_UNIT},
    program::{FailedToReduce, GoblinError, GoblinResult, OrderToInsert, ReduceOrderPacket},
    quantities::{
        AdjustedQuoteLots, BaseLots, BaseLotsPerBaseUnit, QuoteLots, QuoteLotsPerBaseUnit, Ticks,
        WrapperU64,
    },
};

use super::{
    adjusted_quote_lot_budget_post_fee_adjustment_for_buys,
    adjusted_quote_lot_budget_post_fee_adjustment_for_sells, compute_fee, inner_indices,
    process_resting_orders, BitmapGroup, InflightOrder, InnerIndex, ListKey, ListSlot, MarketState,
    MatchingEngineResponse, MutableBitmap, OrderId, OrderPacket, OrderPacketMetadata, OuterIndex,
    Side, SlotRestingOrder, SlotStorage, TickIndices, TraderState,
};
use alloc::vec::Vec;

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
    slot_storage: &mut SlotStorage,
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
        fail_on_cross,
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
        } else if let Some(ticks) = check_for_cross(
            slot_storage,
            market_state,
            side,
            *price_in_ticks,
            current_block,
            current_unix_timestamp,
        ) {
            if *fail_on_cross {
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
        let resting_order = match_order(
            slot_storage,
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
            get_best_available_order_id(slot_storage, &order_packet, last_order);

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
                            matching_engine_response.use_free_quote_lots(quote_lots_free_to_use);
                        }
                        Side::Ask => {
                            let base_lots_free_to_use =
                                resting_order.num_base_lots.min(trader_state.base_lots_free);
                            trader_state.use_free_base_lots(base_lots_free_to_use);
                            trader_state.lock_base_lots(resting_order.num_base_lots);
                            matching_engine_response.post_base_lots(resting_order.num_base_lots);
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
                trader_state.deposit_free_quote_lots(matching_engine_response.num_quote_lots_out);
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

/// Match the inflight order with crossing resting orders of the opposite side.
///
/// Returns a SlotRestingOrder which for
/// - Limit case: should be posted as a resting order
/// - IOC case: is used to validate fill conditions
///
fn match_order(
    slot_storage: &mut SlotStorage,
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
                                    order_id.price_in_ticks * TICK_SIZE_IN_QUOTE_LOTS_PER_BASE_UNIT,
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

        let num_adjusted_quote_lots_quoted =
            order_id.price_in_ticks * TICK_SIZE_IN_QUOTE_LOTS_PER_BASE_UNIT * num_base_lots_quoted;

        // Use matched_base_lots and matched_adjusted_quote_lots to update the
        // inflight order and trader state
        let (matched_base_lots, matched_adjusted_quote_lots) = {
            // Check if the inflight order's budget is exhausted
            // Compare inflight order's budgets with quoted lots
            let has_remaining_adjusted_quote_lots =
                num_adjusted_quote_lots_quoted <= inflight_order.adjusted_quote_lot_budget;
            let has_remaining_base_lots = num_base_lots_quoted <= inflight_order.base_lot_budget;

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

    process_resting_orders(slot_storage, market_state, opposite_side, &mut handle_match);

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
    slot_storage: &SlotStorage,
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
        let bitmap_group = BitmapGroup::new_from_slot(slot_storage, current_outer_index);

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
                let relative_index_of_last_order = last_order.unwrap().order_id.resting_order_index;
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
    slot_storage: &mut SlotStorage,
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
    process_resting_orders(slot_storage, market_state, opposite_side, &mut handle_cross);

    crossing_tick
}

pub struct ReduceOrderInnerResponse {
    pub matching_engine_response: MatchingEngineResponse,
    pub should_remove_order_from_book: bool,
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
