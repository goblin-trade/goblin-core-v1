use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
        update::{update_marker::UpdateMarker, Increase},
    },
    goblin_error::GoblinError,
    quantities::{BaseLots, InnerPos},
    settlement::CheckedAdd,
    state::{
        bitmap::alias::InnerBitmap,
        resting_order::{preimage::RestingOrderPreimage, RestingOrder},
        SlotKey,
    },
};

impl UpdateMarker for Increase {
    fn update_resting_order<'a, M, B, Q, In>(
        resting_order_key: &SlotKey<RestingOrderPreimage<M, B, Q>>,
        base_lots: BaseLots,
        _inner_pos: InnerPos,
        resting_order_state: &mut RestingOrder,
        _inner_bitmap_state: &mut InnerBitmap,
    ) -> Result<BaseLots, GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher,
    {
        resting_order_state.base_lots = resting_order_state
            .base_lots
            .checked_add(base_lots)
            .ok_or(GoblinError::Overflow)?;
        resting_order_key.store(&resting_order_state);

        Ok(base_lots)
    }
}
