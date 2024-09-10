use stylus_sdk::{alloy_primitives::Address, block};

use crate::{
    parameters::{BASE_LOTS_PER_BASE_UNIT, TICK_SIZE_IN_QUOTE_LOTS_PER_BASE_UNIT},
    quantities::{AdjustedQuoteLots, BaseLots, QuoteLots, Ticks, WrapperU64},
    state::{
        order::resting_order::SlotRestingOrder, InflightOrder, MarketState, MatchingEngineResponse,
        OrderPacket, OrderPacketMetadata, Side, SlotStorage, TraderState,
    },
};

use super::{
    adjusted_quote_lot_budget_post_fee_adjustment_for_buys,
    adjusted_quote_lot_budget_post_fee_adjustment_for_sells, check_for_cross,
    get_best_available_order_id, match_order, round_adjusted_quote_lots_down,
    round_adjusted_quote_lots_up, OrderToInsert,
};

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
