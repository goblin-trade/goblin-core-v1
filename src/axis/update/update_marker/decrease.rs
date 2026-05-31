use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        market::market_marker::MarketMarker,
        occupancy::occupancy_marker::OccupancyMarker,
        token::token_marker::TokenMarker,
        update::{update_marker::UpdateMarker, Decrease},
    },
    goblin_error::GoblinError,
    quantities::{BaseLots, InnerPos},
    state::{bitmap::alias::InnerBitmap, resting_order::preimage::RestingOrderPreimage, SlotKey},
    types::Address,
};

impl<In> UpdateMarker<In> for Decrease
where
    In: LegMatcher,
{
    fn update_resting_order<'a, M, B, Q, Oc>(
        msg_sender: &Address,
        base_lots: BaseLots,
        inner_pos: InnerPos,
        inner_bitmap_state: &mut InnerBitmap,
        key: &SlotKey<RestingOrderPreimage<M, B, Q>>,
    ) -> Result<BaseLots, GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        Oc: OccupancyMarker,
    {
        Oc::decrease(msg_sender, base_lots, inner_pos, inner_bitmap_state, key)
    }
}
