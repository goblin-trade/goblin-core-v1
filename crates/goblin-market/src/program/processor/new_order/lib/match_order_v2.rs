use stylus_sdk::alloy_primitives::Address;

use crate::{
    quantities::AdjustedQuoteLots,
    state::{
        order::resting_order::SlotRestingOrder, remove::order_id_remover::OrderIdRemover,
        ArbContext, InflightOrder, MarketState, Side,
    },
};

use super::ExpiryChecker;

pub struct MatchOrderManager {
    remover: OrderIdRemover,
}

impl MatchOrderManager {
    // pub fn new(ctx: &mut ArbContext, side: Side, market_state: &mut MarketState) -> Option<Self> {
    //     OrderIdRemover::new_for_matching(ctx, side, market_state)
    //         .map(|remover| MatchOrderManager { remover })
    // }

    // pub fn next_order_id(&mut self) {
    //     self.remover.order_id_is_active(ctx, order_id)
    // }
}

/// Match the inflight order with crossing resting orders of the opposite side.
///
/// Returns a SlotRestingOrder which for
/// - Limit case: should be posted as a resting order
/// - IOC case: ~~is used to validate fill conditions~~ not used anywhere
///
pub fn match_order_v2(
    ctx: &mut ArbContext,
    expiry_checker: &mut ExpiryChecker,
    market_state: &mut MarketState,
    inflight_order: &mut InflightOrder,
    taker_address: Address,
) -> Option<SlotRestingOrder> {
    let mut total_matched_adjusted_quote_lots = AdjustedQuoteLots::ZERO;
    while inflight_order.in_progress() {
        // Read opposite side orders starting from the centre
        // Write an interator in /orderbook to loop through active bits
        // If the resting order is exhausted then this cached order is cleared
        // and we move on
    }
    None
}
