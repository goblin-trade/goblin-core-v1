use crate::{
    program::{
        match_order,
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
    let side = order_packet.side;

    if order_packet.is_invalid(ctx, expiry_checker) {
        return None;
    }

    let mut inflight_order = order_packet.get_inflight_order();

    match_order(
        ctx,
        expiry_checker,
        market_state,
        &mut inflight_order,
        order_packet.trader,
    )?;

    None
}
