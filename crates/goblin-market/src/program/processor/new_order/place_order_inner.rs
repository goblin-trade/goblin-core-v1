use stylus_sdk::{alloy_primitives::Address, block};

use crate::{
    parameters::BASE_LOTS_PER_BASE_UNIT,
    program::types::{
        matching_engine_response::MatchingEngineResponse,
        order_packet::{OrderPacket, OrderPacketMetadata},
    },
    quantities::{AdjustedQuoteLots, BaseLots, QuoteLots, Ticks, WrapperU64},
    state::{
        order::resting_order::SlotRestingOrder, InflightOrder, MarketState, Side, SlotStorage,
        TraderState,
    },
};

use super::{
    adjusted_quote_lot_budget_post_fee_adjustment_for_buys,
    adjusted_quote_lot_budget_post_fee_adjustment_for_sells, check_for_cross, compute_quote_lots,
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
    // - matching_engine_response for PostOnly will be initially empty. Lots are posted
    // later on when the order is posted to the book.
    // - matching_engine_response for IOC and limit will contain the tokens transferred
    // in and transferred out during matching.
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
            // In an IOC bid, we match against asks.
            // Quote tokens are traded for base tokens.
            Side::Bid => MatchingEngineResponse::new_from_buy(
                matched_quote_lots,               // quote token input
                inflight_order.matched_base_lots, // base token output
            ),

            // In an IOC ask, we match against bids.
            // Base tokens are traded for quote tokens.
            Side::Ask => MatchingEngineResponse::new_from_sell(
                inflight_order.matched_base_lots, // base token input
                matched_quote_lots,               // quote token output
            ),
        };

        // EMIT FillSummary

        // TODO deduct free lots from trader state here itself?

        if let OrderPacket::ImmediateOrCancel {
            min_base_lots_to_fill,
            min_quote_lots_to_fill,
            ..
        } = order_packet
        {
            // For IOC orders, if the order's minimum fill requirements are not met, then
            // the order is voided. Return None.
            if !matching_engine_response
                .verify_minimum_lots_filled(*min_base_lots_to_fill, *min_quote_lots_to_fill)
            {
                return None;
            }
        }

        (resting_order, matching_engine_response)
    };

    let mut order_to_insert: Option<OrderToInsert> = None;

    if !order_packet.is_ioc() && resting_order.num_base_lots > BaseLots::ZERO {
        // PostOnly and limit case
        // - Find best available order ID to place order
        // - Update trader state and matching engine response

        if let Some(order_id) = get_best_available_order_id(slot_storage, &order_packet, last_order)
        {
            // Queue resting order for insertion, update states and matching engine response
            // This happens only in limit and post-only case, not IOC
            order_to_insert = Some(OrderToInsert {
                order_id,
                resting_order,
            });

            // Update trader state and matching engine response
            match side {
                Side::Bid => {
                    // Quote lots are posted in a bid order

                    // Lock up tokens needed to post the order and use up available free lots
                    // from the trader state. If free lots fall short then they will be transferred
                    // in later. MatchingEngineResponse is used to calculate the outstanding lots.

                    // The number of lots required to post the resting order on the book
                    let quote_lots_to_lock =
                        compute_quote_lots(order_id.price_in_ticks, resting_order.num_base_lots);

                    // Number of quote lots available to post the order
                    let quote_lots_free_to_use =
                        quote_lots_to_lock.min(trader_state.quote_lots_free);

                    trader_state.lock_quote_lots(&mut matching_engine_response, quote_lots_to_lock);
                    trader_state
                        .use_free_quote_lots(&mut matching_engine_response, quote_lots_free_to_use);
                }
                Side::Ask => {
                    // Base lots are posted in an ask order
                    let base_lots_free_to_use =
                        resting_order.num_base_lots.min(trader_state.base_lots_free);

                    trader_state
                        .lock_base_lots(&mut matching_engine_response, resting_order.num_base_lots);
                    trader_state
                        .use_free_base_lots(&mut matching_engine_response, base_lots_free_to_use);
                }
            }

            // EMIT Place
            // EMIT TimeInForce if this is a time in force order
        } else {
            // No space for order, exit
            // Multiple orders behavior is handled outside
            // Currently the entire TX fails
            return None;
        }
    }

    // if let OrderPacket::ImmediateOrCancel {
    //     min_base_lots_to_fill,
    //     min_quote_lots_to_fill,
    //     ..
    // } = order_packet
    // {
    //     // For IOC orders, if the order's minimum fill requirements are not met, then
    //     // the order is voided. Return None.
    //     if !matching_engine_response
    //         .verify_minimum_lots_filled(*min_base_lots_to_fill, *min_quote_lots_to_fill)
    //     {
    //         return None;
    //     }
    // } else if resting_order.num_base_lots > BaseLots::ZERO {
    //     // PostOnly and limit case
    //     // - Find best available order ID to place order
    //     // - Update trader state and matching engine response

    //     if let Some(order_id) = get_best_available_order_id(slot_storage, &order_packet, last_order)
    //     {
    //         // Queue resting order for insertion, update states and matching engine response
    //         // This happens only in limit and post-only case, not IOC
    //         order_to_insert = Some(OrderToInsert {
    //             order_id,
    //             resting_order,
    //         });

    //         // Update trader state and matching engine response
    //         match side {
    //             Side::Bid => {
    //                 // Quote lots are posted in a bid order

    //                 // Lock up tokens needed to post the order and use up available free lots
    //                 // from the trader state. If free lots fall short then they will be transferred
    //                 // in later. MatchingEngineResponse is used to calculate the outstanding lots.

    //                 // The number of lots required to post the resting order on the book
    //                 let quote_lots_to_lock =
    //                     compute_quote_lots(order_id.price_in_ticks, resting_order.num_base_lots);

    //                 // Number of quote lots available to post the order
    //                 let quote_lots_free_to_use =
    //                     quote_lots_to_lock.min(trader_state.quote_lots_free);

    //                 trader_state.lock_quote_lots(&mut matching_engine_response, quote_lots_to_lock);
    //                 trader_state
    //                     .use_free_quote_lots(&mut matching_engine_response, quote_lots_free_to_use);
    //             }
    //             Side::Ask => {
    //                 // Base lots are posted in an ask order
    //                 let base_lots_free_to_use =
    //                     resting_order.num_base_lots.min(trader_state.base_lots_free);

    //                 trader_state
    //                     .lock_base_lots(&mut matching_engine_response, resting_order.num_base_lots);
    //                 trader_state
    //                     .use_free_base_lots(&mut matching_engine_response, base_lots_free_to_use);
    //             }
    //         }

    //         // EMIT Place
    //         // EMIT TimeInForce if this is a time in force order
    //     } else {
    //         // No space for order, exit
    //         // Multiple orders behavior is handled outside
    //         // Currently the entire TX fails
    //         return None;
    //     }
    // }

    // Limit and post-only branch ends
    // Update the trader state and matching engine response

    // Check if trader has free lots
    match side {
        Side::Bid => {
            // Replace num_quote_lots() with quote lots in?
            // quote_lots_out will be zero for bids
            let quote_lots_free_to_use = trader_state
                .quote_lots_free
                .min(matching_engine_response.num_quote_lots()); // This will be 0 for post-only case

            trader_state.use_free_quote_lots(&mut matching_engine_response, quote_lots_free_to_use);
        }
        Side::Ask => {
            let base_lots_free_to_use = trader_state
                .base_lots_free
                .min(matching_engine_response.num_base_lots());

            trader_state.use_free_base_lots(&mut matching_engine_response, base_lots_free_to_use);
        }
    }

    // If `use_only_deposited_funds` is true, credit the output tokens to trader state
    // and set output tokens to 0.
    if order_packet.no_deposit_or_withdrawal() {
        trader_state.deposit_output_into_free_lots(&mut matching_engine_response, side);

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
