use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
        update::{update_marker::UpdateMarker, Decrease},
    },
    goblin_error::GoblinError,
    quantities::{BaseLots, InnerPos},
    state::{bitmap::alias::InnerBitmap, resting_order::preimage::RestingOrderPreimage, KeyValue},
};

impl<In> UpdateMarker<In> for Decrease
where
    In: LegMatcher,
{
    fn update_resting_order<'a, M, B, Q>(
        base_lots: BaseLots,
        inner_pos: InnerPos,
        inner_bitmap_state: &mut InnerBitmap,
        key_value: &mut KeyValue<RestingOrderPreimage<M, B, Q>>,
    ) -> Result<BaseLots, GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
    {
        let stored_base_lots = &mut key_value.value.base_lots;
        let reduced_lots = if *stored_base_lots > base_lots {
            *stored_base_lots -= base_lots;
            key_value.store();

            base_lots
        } else {
            inner_bitmap_state.deactivate(inner_pos);

            *stored_base_lots
        };

        Ok(reduced_lots)
    }
}
