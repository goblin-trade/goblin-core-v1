use stylus_sdk::alloy_primitives::Address;

use crate::{
    parameters::{BASE_LOTS_PER_BASE_UNIT, TICK_SIZE_IN_QUOTE_LOTS_PER_BASE_UNIT},
    quantities::{AdjustedQuoteLots, BaseLots, QuoteLotsPerBaseUnit},
    state::{
        order::{order_id::OrderId, resting_order::SlotRestingOrder},
        process_resting_orders::process_resting_orders,
        ArbContext, InflightOrder, MarketState, Side, TraderState,
    },
};

use super::{compute_fee, round_adjusted_quote_lots_up};

/// Match the inflight order with crossing resting orders of the opposite side.
///
/// Returns a SlotRestingOrder which for
/// - Limit case: should be posted as a resting order
/// - IOC case: is used to validate fill conditions
///
pub fn match_order(
    slot_storage: &mut ArbContext,
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
                            slot_storage: &mut ArbContext| {
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
        if resting_order.is_expired(current_block, current_unix_timestamp_in_seconds) {
            resting_order.reduce_order(
                &mut maker_state,
                opposite_side,
                order_id.price_in_ticks,
                BaseLots::MAX,
                true,
                false,
            );
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

                    resting_order.reduce_order(
                        &mut maker_state,
                        opposite_side,
                        order_id.price_in_ticks,
                        BaseLots::MAX,
                        false,
                        false,
                    );

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

                    resting_order.reduce_order(
                        &mut maker_state,
                        opposite_side,
                        order_id.price_in_ticks,
                        base_lots_removed,
                        false,
                        false,
                    );

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
            maker_state.write_to_slot(slot_storage, resting_order.trader_address);
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
                // TODO remove clear_order() function. No need to clear closed orders,
                // simply clear its corresponding bitmap slot.
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
