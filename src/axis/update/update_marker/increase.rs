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
    state::{bitmap::alias::InnerBitmap, resting_order::preimage::RestingOrderPreimage, KeyValue},
};

impl<In> UpdateMarker<In> for Increase
where
    In: LegMatcher,
{
    fn update_resting_order<'a, M, B, Q>(
        base_lots: BaseLots,
        _inner_pos: InnerPos,
        _inner_bitmap_state: &mut InnerBitmap,
        key_value: &mut KeyValue<RestingOrderPreimage<M, B, Q>>,
    ) -> Result<BaseLots, GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
    {
        let stored_base_lots = &mut key_value.value.base_lots;
        *stored_base_lots = stored_base_lots
            .checked_add(base_lots)
            .ok_or(GoblinError::Overflow)?;

        key_value.store();

        Ok(base_lots)
    }
}
