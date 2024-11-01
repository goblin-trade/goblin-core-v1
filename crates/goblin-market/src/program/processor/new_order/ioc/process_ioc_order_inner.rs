use crate::{
    program::{
        match_order, match_order_v2,
        types::{matching_engine_response::MatchingEngineResponse, order_packet::OrderPacket},
        ExpiryChecker,
    },
    quantities::{BaseLots, QuoteLots, Ticks},
    state::{ArbContext, MarketState, Side, TraderState},
};

use super::ImmediateOrCancelOrderPacket;

pub fn process_ioc_order_inner(
    ctx: &mut ArbContext,
    expiry_checker: &mut ExpiryChecker,
    market_state: &mut MarketState,
    trader_state: &mut TraderState,
    order_packet: &mut ImmediateOrCancelOrderPacket,
) -> Option<MatchingEngineResponse> {
    if order_packet.is_invalid(ctx, expiry_checker) {
        return None;
    }

    let mut inflight_order = order_packet.get_inflight_order();

    let resting_order = match_order_v2(
        ctx,
        market_state,
        &mut inflight_order,
        expiry_checker,
        order_packet.trader,
    )?;

    // Update trader state and generate matching engine response
    let matching_engine_response = trader_state.take_order(
        order_packet.side,
        inflight_order.matched_quote_lots(),
        inflight_order.matched_base_lots,
    );

    // EMIT FillSummary

    // Enforce minimum fill condition for IOC orders
    if !matching_engine_response.verify_minimum_lots_filled(
        order_packet.min_base_lots_to_fill,
        order_packet.min_quote_lots_to_fill,
    ) {
        return None;
    }

    None
}
