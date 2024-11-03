use stylus_sdk::alloy_primitives::Address;

use crate::{
    parameters::{BASE_LOTS_PER_BASE_UNIT, TICK_SIZE_IN_QUOTE_LOTS_PER_BASE_UNIT},
    quantities::{AdjustedQuoteLots, BaseLots, QuoteLotsPerBaseUnit},
    state::{
        order::{order_id::OrderId, resting_order::SlotRestingOrder},
        remove::{IOrderSequentialRemover, OrderSequentialRemover},
        ArbContext, InflightOrder, MarketState, SelfTradeBehavior, Side, TraderState,
    },
};

use super::{compute_fee, round_adjusted_quote_lots_up, ExpiryChecker};

/// Read resting orders for a side begininning from the centre and moving outwards.
/// next() returns the next smallest order and removes the previously returned one.
///
/// This is a wrapper around OrderSequentialRemover which also reads slot resting orders.
struct SlabReader<'a> {
    inner: OrderSequentialRemover<'a>,
}

impl<'a> SlabReader<'a> {
    pub fn new(opposite_side: Side, market_state: &'a mut MarketState) -> Self {
        // let unclaimed_quote_lot_fees = &mut market_state.unclaimed_quote_lot_fees;
        let (best_market_price, outer_index_count) =
            market_state.best_market_price_and_outer_index_count(opposite_side);

        SlabReader {
            inner: OrderSequentialRemover::new(opposite_side, best_market_price, outer_index_count),
        }
    }

    fn next(&mut self, ctx: &mut ArbContext) -> Option<(OrderId, SlotRestingOrder)> {
        if let Some(order_id) = self.inner.next(ctx) {
            let resting_order = SlotRestingOrder::new_from_slot(ctx, order_id);
            Some((order_id, resting_order))
        } else {
            None
        }
    }
}

/// Match an inflight order against the opposite side slab.
///
/// An inflight order has a base lot budget that is depleted as the order
/// is matched with resting orders on the opposite side book. Control variables
/// like `limit_price_in_ticks` and `match_limit` are used to restrict matching.
pub fn match_order_v2(
    ctx: &mut ArbContext,
    market_state: &mut MarketState,
    inflight_order: &mut InflightOrder,
    expiry_checker: &mut ExpiryChecker,
    taker_address: Address,
) -> Option<SlotRestingOrder> {
    let mut total_matched_adjusted_quote_lots = AdjustedQuoteLots::ZERO;
    let opposite_side = inflight_order.side.opposite();
    let mut slab_reader = SlabReader::new(opposite_side, market_state);

    while inflight_order.in_progress() {
        if let Some((OrderId { price_in_ticks, .. }, mut resting_order)) = slab_reader.next(ctx) {
            let num_base_lots_quoted = resting_order.num_base_lots;

            // 0. Not crossed case
            // Not crossed if limit price of inflight order is closer to the centre
            // than price of the best available order
            let not_crossed = inflight_order
                .limit_price_in_ticks
                .is_closer_to_center(opposite_side, price_in_ticks);

            if not_crossed {
                break;
            }

            let mut maker_state = TraderState::read_from_slot(ctx, resting_order.trader_address);

            // 1. Resting order expired case
            // Free up tokens for maker and continue to read the next order.
            // We do not write the cleared resting order to slot. The sequential remover
            // will deactivate it when next() is called.
            if expiry_checker.is_expired(
                ctx,
                resting_order.track_block,
                resting_order.last_valid_block_or_unix_timestamp_in_seconds,
            ) {
                maker_state.cancel_order_and_claim_funds(
                    &mut resting_order,
                    opposite_side,
                    price_in_ticks,
                    false, // don't claim funds for maker
                );
                maker_state.write_to_slot(ctx, resting_order.trader_address);
                inflight_order.match_limit -= 1;
                continue;
            }

            // 2. Self trade case
            if taker_address == resting_order.trader_address {
                match inflight_order.self_trade_behavior {
                    SelfTradeBehavior::Abort => {
                        // Return None to abort the transaction
                        return None;
                    }
                    SelfTradeBehavior::CancelProvide => {
                        // Cancel the resting order without charging fees.
                        maker_state.cancel_order_and_claim_funds(
                            &mut resting_order,
                            opposite_side,
                            price_in_ticks,
                            false, // don't claim funds for maker
                        );
                        inflight_order.match_limit -= 1;
                        maker_state.write_to_slot(ctx, resting_order.trader_address);
                        continue;
                    }
                    SelfTradeBehavior::DecrementTake => {
                        // Match against the maker order, but don't add fees
                        // Similar matching logic is used later, but here the amount matched is
                        // not added to total_matched_adjusted_quote_lots
                        let base_lots_to_remove = inflight_order
                            .base_lots_available_to_match(price_in_ticks)
                            .min(num_base_lots_quoted); // Cap base lots to the amount quoted

                        let adjusted_quote_lots_to_remove = price_in_ticks
                            * TICK_SIZE_IN_QUOTE_LOTS_PER_BASE_UNIT
                            * base_lots_to_remove;

                        inflight_order.process_decrement_take(
                            adjusted_quote_lots_to_remove,
                            base_lots_to_remove,
                        );

                        resting_order.reduce_order(
                            &mut maker_state,
                            opposite_side,
                            price_in_ticks,
                            base_lots_to_remove,
                            false,
                            false,
                        );
                        // TODO resting order not written?

                        // TODO why not use InflightOrder::process_match()?
                        // Decrement base lot and adjusted quote lot budgets
                        // inflight_order.base_lot_budget = inflight_order
                        //     .base_lot_budget
                        //     .saturating_sub(base_lots_to_remove);

                        // inflight_order.adjusted_quote_lot_budget = inflight_order
                        //     .adjusted_quote_lot_budget
                        //     .saturating_sub(adjusted_quote_lots_to_remove);

                        // // Self trades will count towards the match limit
                        // inflight_order.match_limit -= 1;

                        maker_state.write_to_slot(ctx, resting_order.trader_address);
                        continue;
                    }
                }
            }
            let num_adjusted_quote_lots_quoted =
                price_in_ticks * TICK_SIZE_IN_QUOTE_LOTS_PER_BASE_UNIT * num_base_lots_quoted;

            // Use matched_base_lots and matched_adjusted_quote_lots to update the
            // inflight order and trader state
            let (matched_base_lots, matched_adjusted_quote_lots) = {
                // Check if the inflight order's budget is exhausted
                // Compare inflight order's budgets with quoted lots
                let has_remaining_adjusted_quote_lots =
                    num_adjusted_quote_lots_quoted <= inflight_order.adjusted_quote_lot_budget;
                let has_remaining_base_lots =
                    num_base_lots_quoted <= inflight_order.base_lot_budget;

                // Inflight order has sufficient budget to consume the quote.
                // Clear the resting order.
                if has_remaining_base_lots && has_remaining_adjusted_quote_lots {
                    // TODO remove clear_order() function. No need to clear closed orders,
                    // simply clear its corresponding bitmap slot.
                    resting_order.clear_order();

                    // Entire quote is matched
                    (num_base_lots_quoted, num_adjusted_quote_lots_quoted)
                } else {
                    // If the order's budget is exhausted, we match as much as we can
                    let base_lots_to_remove =
                        inflight_order.base_lots_available_to_match(price_in_ticks);
                    let adjusted_quote_lots_to_remove = price_in_ticks
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
            maker_state.write_to_slot(ctx, resting_order.trader_address);
        } else {
            break;
        }
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
