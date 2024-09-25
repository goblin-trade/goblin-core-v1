use stylus_sdk::{alloy_primitives::Address, block};

use crate::{
    program::types::{
        matching_engine_response::MatchingEngineResponse,
        order_packet::{OrderPacket, OrderPacketMetadata},
    },
    quantities::{BaseLots, QuoteLots, Ticks},
    state::{ArbContext, MarketState, Side, TraderState},
};

use super::{check_for_cross, get_best_available_order_id, match_order, OrderToInsert};

/// Try to execute an order packet and place an order
///
/// TODO split into separate functions for each order type.
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
    slot_storage: &mut ArbContext,
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

    // TODO don't load both
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
        // Optimization- since successive orders move away from the centre, we
        // only need to check the first order for cross. Subsequent crossing orders
        // can be moved to the previously calculated non-crossing price.
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
            order_packet.get_resting_order(trader),
            MatchingEngineResponse::default(),
        )
    } else {
        // Limit and IOC branch
        let mut inflight_order = order_packet.get_inflight_order();

        // Use the resting order
        // - Limit case- place a new resting order with this amount
        // - IOC case- value discarded.
        //
        // If match_order() returns None then the `?` unwrap will cause this function
        // to return None too.
        let resting_order = match_order(
            slot_storage,
            market_state,
            &mut inflight_order,
            trader,
            current_block,
            current_unix_timestamp,
        )?;

        // Update trader state and generate matching engine response
        let matching_engine_response = trader_state.take_order(
            side,
            inflight_order.matched_quote_lots(),
            inflight_order.matched_base_lots,
        );

        // EMIT FillSummary

        // Enforce minimum fill condition for IOC orders
        if let OrderPacket::ImmediateOrCancel {
            min_base_lots_to_fill,
            min_quote_lots_to_fill,
            ..
        } = order_packet
        {
            if !matching_engine_response
                .verify_minimum_lots_filled(*min_base_lots_to_fill, *min_quote_lots_to_fill)
            {
                return None;
            }
        }

        (resting_order, matching_engine_response)
    };

    let mut order_to_insert: Option<OrderToInsert> = None;
    if (order_packet.is_post_only() || order_packet.is_limit())
        && resting_order.num_base_lots > BaseLots::ZERO
    {
        if let Some(order_id) = get_best_available_order_id(slot_storage, &order_packet, last_order)
        {
            // Queue resting order for insertion
            order_to_insert = Some(OrderToInsert {
                order_id,
                resting_order,
            });

            // Update trader state and matching engine response
            trader_state.make_order(
                &mut matching_engine_response,
                side,
                order_id.price_in_ticks,
                resting_order.num_base_lots,
            );

            // EMIT Place
            // EMIT TimeInForce if this is a time in force order
        } else {
            // No space for order, exit
            // Multiple orders behavior is handled outside
            // Currently the entire TX fails
            return None;
        }
    }
    // Limit and post-only branch ends

    // If `use_only_deposited_funds` is true, credit the output tokens to trader state
    // and set output tokens to 0.
    if order_packet.no_deposit_or_withdrawal() {
        // Credit output lots to trader state and set them to zero in matching engine response.
        // No need to check verify_no_withdrawal() now
        trader_state.deposit_output_into_free_lots(&mut matching_engine_response, side);

        if !matching_engine_response.verify_no_deposit() {
            return None;
        }
    }

    // Fully consumed limit order and empty post-only order will have order_to_insert = None
    Some((order_to_insert, matching_engine_response))
}
