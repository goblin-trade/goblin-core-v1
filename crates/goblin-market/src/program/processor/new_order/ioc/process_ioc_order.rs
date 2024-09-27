use stylus_sdk::alloy_primitives::Address;

use crate::{
    program::{ExpiryChecker, GoblinResult},
    state::{ArbContext, ContextActions, MarketState, TraderState},
    GoblinMarket,
};

use super::ImmediateOrCancelOrderPacket;

pub fn process_ioc_order(order_packet: &mut ImmediateOrCancelOrderPacket) -> GoblinResult<()> {
    let ctx = &mut ArbContext::new();
    let mut expiry_checker = ExpiryChecker::new();

    let mut market_state = MarketState::read_from_slot(ctx);
    let mut trader_state = TraderState::read_from_slot(ctx, order_packet.trader);

    let side = order_packet.side;

    // fail_silently_on_insufficient_funds is always false for ImmediateOrCancel
    // Skip check

    // call process_ioc_order_inner
    // Obtain tokens to transfer in and tokens to transfer out

    // No object returned. Skip resting order insertion

    // Perform token transfers
    Ok(())
}
