use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
        update::{update_marker::UpdateMarker, Decrease},
    },
    goblin_error::GoblinError,
    quantities::{BaseLots, InnerPos},
    state::{
        bitmap::alias::InnerBitmap,
        resting_order::{preimage::RestingOrderPreimage, RestingOrder},
        SlotKey,
    },
};

impl UpdateMarker for Decrease {
    fn update_resting_order<'a, M, B, Q, In>(
        resting_order_key: &SlotKey<RestingOrderPreimage<M, B, Q>>,
        base_lots: BaseLots,
        inner_pos: InnerPos,
        resting_order_state: &mut RestingOrder,
        inner_bitmap_state: &mut InnerBitmap,
    ) -> Result<BaseLots, GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher,
    {
        let reduced_lots = if resting_order_state.base_lots > base_lots {
            resting_order_state.base_lots -= base_lots;
            resting_order_key.store(&resting_order_state);

            base_lots
        } else {
            inner_bitmap_state.deactivate(inner_pos);

            resting_order_state.base_lots
        };

        Ok(reduced_lots)
    }
}
