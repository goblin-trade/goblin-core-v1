use stylus_sdk::alloy_primitives::Address;

use crate::{
    parameters::{BASE_LOTS_PER_BASE_UNIT, TICK_SIZE_IN_QUOTE_LOTS_PER_BASE_UNIT},
    program::{GoblinError, GoblinResult},
    quantities::{AdjustedQuoteLots, BaseLots, QuoteLotsPerBaseUnit, Ticks},
    state::{InflightOrder, MarketState, RestingOrder, Side, SlotStorage, TraderState},
};

use super::{
    inner_indices, BitmapGroup, InnerIndex, ListKey, ListSlot, OrderId, RestingOrderIndex,
    SlotRestingOrder,
};

/// Loops through subsequent resting orders, applying a lambda function on each.
/// The loop moves to the next resting order if lambda function closes the current resting
/// order and returns ContinueLooping.
/// Returns the OrderId of the best active resting order remaining after the lambda function
/// is applied.
pub fn process_resting_orders(
    slot_storage: &mut SlotStorage,
    market_state: &mut MarketState,
    num_ticks: Ticks,
    side: Side,
    current_block: u32,
    current_unix_timestamp_in_seconds: u32,
    lambda_function: fn(
        resting_order: &mut SlotRestingOrder,
        num_ticks: Ticks,
        resting_order_price: Ticks,
        side: Side,
        current_block: u32,
        current_unix_timestamp_in_seconds: u32,
    ) -> LambdaResult,
) -> GoblinResult<Option<OrderId>> {
    let mut outer_index_count = market_state.outer_index_length(side);
    let mut price_in_ticks = market_state.best_price(side);
    let mut previous_inner_index = Some(price_in_ticks.inner_index());
    let mut slot_index = (outer_index_count - 1) / 16;
    let mut relative_index = (outer_index_count - 1) % 16;

    // let mut stop_reads: Option<bool> = None;
    let mut lambda_result = LambdaResult::ContinueLoop;

    // 1. Loop through index slots
    loop {
        let list_key = ListKey { index: slot_index };
        let mut list_slot = ListSlot::new_from_slot(slot_storage, list_key);
        let mut pending_list_slot_write = false;

        // 2. Loop through bitmap groups using relative index
        loop {
            let outer_index = list_slot.get(relative_index as usize);
            let mut bitmap_group = BitmapGroup::new_from_slot(slot_storage, outer_index);

            let mut pending_bitmap_group_write = false;

            // 3. Loop through bitmaps
            for i in inner_indices(side, previous_inner_index) {
                let inner_index = InnerIndex::new(i);
                price_in_ticks = Ticks::from_indices(outer_index, inner_index);
                let mut bitmap = bitmap_group.get_bitmap_mut(&inner_index);

                // 4. Loop through resting orders in the bitmap
                for j in 0..8 {
                    let resting_order_index = RestingOrderIndex::new(j);
                    let order_present = bitmap.order_present(resting_order_index);

                    if order_present {
                        let order_id = OrderId {
                            price_in_ticks,
                            resting_order_index,
                        };

                        if lambda_result != LambdaResult::ContinueLoop {
                            if pending_bitmap_group_write {
                                bitmap_group.write_to_slot(slot_storage, &outer_index);
                            }
                            if pending_list_slot_write {
                                list_slot.write_to_slot(slot_storage, &list_key);
                            }
                            market_state.set_best_price(side, price_in_ticks);
                            market_state.set_outer_index_length(side, outer_index_count);

                            return match lambda_result {
                                LambdaResult::ReturnNone => Ok(None),
                                LambdaResult::ReturnOrderId => Ok(Some(order_id)),
                                LambdaResult::ContinueLoop => unreachable!(),
                            };
                        }

                        let mut resting_order =
                            SlotRestingOrder::new_from_slot(slot_storage, order_id);

                        // lambda_result = lambda_function(&mut resting_order);
                        lambda_result = lambda_function(
                            &mut resting_order,
                            num_ticks,
                            price_in_ticks,
                            side,
                            current_block,
                            current_unix_timestamp_in_seconds,
                        );

                        resting_order.write_to_slot(slot_storage, &order_id)?;

                        // The input amount is consumed, exit.
                        // Traversed Bitmap groups and ListSlots have been written already
                        if resting_order.size() != 0 {
                            market_state.set_best_price(side, price_in_ticks);
                            market_state.set_outer_index_length(side, outer_index_count);

                            return match lambda_result {
                                LambdaResult::ReturnNone => Ok(None),
                                LambdaResult::ReturnOrderId => Ok(Some(order_id)),
                                LambdaResult::ContinueLoop => unreachable!(),
                            };
                        }

                        bitmap.clear(&resting_order_index);
                        pending_bitmap_group_write = true;
                    }
                }
            }
            // Previous inner index is only used for the first active tick
            if previous_inner_index.is_some() {
                previous_inner_index = None;
            }

            // Empty bitmap group written to slot
            bitmap_group.write_to_slot(slot_storage, &outer_index);

            list_slot.clear_index(&list_key);
            pending_list_slot_write = true;
            outer_index_count -= 1;

            if relative_index == 0 {
                break;
            }
            relative_index -= 1;
        }

        // All orders for the slot index have been purged
        // Empty list slot written to slot
        list_slot.write_to_slot(slot_storage, &list_key);

        if slot_index == 0 {
            break;
        }
        // Move to the next ListSlot. Reset the relative index.
        slot_index -= 1;
        relative_index = 15;
    }

    Ok(None)
}

#[derive(PartialEq, Eq)]
pub enum LambdaResult {
    ContinueLoop,
    ReturnNone,
    ReturnOrderId,
}

pub fn order_crosses(
    resting_order: &mut SlotRestingOrder,
    trader_state: &mut TraderState,
    order_id: OrderId,
    side: Side,
    limit_price_in_ticks: Ticks,
    current_block: u32,
    current_unix_timestamp_in_seconds: u32,
) -> LambdaResult {
    let crosses = match side.opposite() {
        Side::Bid => order_id.price_in_ticks >= limit_price_in_ticks,
        Side::Ask => order_id.price_in_ticks <= limit_price_in_ticks,
    };

    if !crosses {
        return LambdaResult::ReturnNone;
    }

    if resting_order.expired(current_block, current_unix_timestamp_in_seconds) {
        resting_order
            .reduce_order(
                trader_state,
                resting_order.trader_address,
                &order_id,
                side.opposite(),
                BaseLots::MAX,
                true,
                false,
            )
            .unwrap();
        return LambdaResult::ContinueLoop;
    }

    return LambdaResult::ReturnOrderId;
}

pub fn match_resting_order(
    inflight_order: &mut InflightOrder,
    total_matched_adjusted_quote_lots: &mut AdjustedQuoteLots,

    resting_order: &mut SlotRestingOrder,
    maker_state: &mut TraderState,
    taker_address: Address,
    order_id: OrderId,
    limit_price_in_ticks: Ticks,
    current_block: u32,
    current_unix_timestamp_in_seconds: u32,
) -> LambdaResult {
    if !inflight_order.in_progress() {
        return LambdaResult::ReturnNone;
    }

    let num_base_lots_quoted = resting_order.num_base_lots;

    let crosses = match inflight_order.side.opposite() {
        Side::Bid => order_id.price_in_ticks >= limit_price_in_ticks,
        Side::Ask => order_id.price_in_ticks <= limit_price_in_ticks,
    };

    if !crosses {
        return LambdaResult::ReturnNone;
    }

    // 1. Resting order expired case
    if resting_order.expired(current_block, current_unix_timestamp_in_seconds) {
        resting_order.clear_order();
        inflight_order.match_limit -= 1;
        return LambdaResult::ContinueLoop;
    }

    // 2. Self trade case
    if taker_address == resting_order.trader_address {
        match inflight_order.self_trade_behavior {
            crate::state::SelfTradeBehavior::Abort => return LambdaResult::ReturnNone,
            crate::state::SelfTradeBehavior::CancelProvide => {
                // Cancel the resting order without charging fees.

                if resting_order
                    .reduce_order(
                        maker_state,
                        taker_address,
                        &order_id,
                        inflight_order.side.opposite(),
                        BaseLots::MAX,
                        false,
                        false,
                    )
                    .is_none()
                {
                    return LambdaResult::ReturnNone;
                }

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

                if resting_order
                    .reduce_order(
                        maker_state,
                        taker_address,
                        &order_id,
                        inflight_order.side.opposite(),
                        base_lots_removed,
                        false,
                        false,
                    )
                    .is_none()
                {
                    return LambdaResult::ReturnNone;
                }

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

                // Update inflight_order.should_terminate, then call continue
                // Input completely exhausted but resting order remains. There is no need
                // to read the next resting order. Setting set_terminate = true will cause
                // the next resting order to be evaluated
                // inflight_order.should_terminate = base_lots_removed < num_base_lots_quoted;

                // Both are exhausted case (==)- we need to read the next item.
                // That is handled correctly
                if base_lots_removed < num_base_lots_quoted {
                    return LambdaResult::ReturnOrderId;
                }
            }
        }

        return LambdaResult::ContinueLoop;
    }

    // General case (non-self trade)

    let num_adjusted_quote_lots_quoted =
        order_id.price_in_ticks * TICK_SIZE_IN_QUOTE_LOTS_PER_BASE_UNIT * num_base_lots_quoted;

    // Use matched_base_lots and matched_adjusted_quote_lots to update the
    // inflight order and trader state
    let (matched_base_lots, matched_adjusted_quote_lots, order_remaining_base_lots) = {
        // Check if the inflight order's budget is exhausted
        // Compare inflight order's budgets with quoted lots
        let has_remaining_adjusted_quote_lots =
            num_adjusted_quote_lots_quoted <= inflight_order.adjusted_quote_lot_budget;
        let has_remaining_base_lots = num_base_lots_quoted <= inflight_order.base_lot_budget;

        // Budget exceeds quote. Clear the resting order.
        // Stop iterating by returning LambdaResult::ReturnOrderId
        if has_remaining_base_lots && has_remaining_adjusted_quote_lots {
            resting_order.clear_order();
            (
                num_base_lots_quoted,
                num_adjusted_quote_lots_quoted,
                BaseLots::ZERO,
            )
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
            // If this clause is reached, we make ensure that the loop terminates
            // as the order has been fully filled
            inflight_order.should_terminate = true;
            (
                base_lots_to_remove,
                adjusted_quote_lots_to_remove,
                resting_order.num_base_lots,
            )
        }
    };

    // Deplete the inflight order's budget by the amount matched
    inflight_order.process_match(matched_adjusted_quote_lots, matched_base_lots);

    // Increment the matched adjusted quote lots for fee calculation
    *total_matched_adjusted_quote_lots += matched_adjusted_quote_lots;

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

    if order_remaining_base_lots == BaseLots::ZERO {
        LambdaResult::ReturnOrderId
    } else {
        LambdaResult::ContinueLoop
    }
}
