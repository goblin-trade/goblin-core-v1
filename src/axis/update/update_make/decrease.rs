use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        market::market_marker::MarketMarker,
        occupancy::occupancy_marker::OccupancyMarker,
        token::token_marker::TokenMarker,
        update::{update_make::UpdateMake, Decrease},
    },
    goblin_error::GoblinError,
    quantities::BaseLots,
    state::{
        bitmap::alias::InnerBitmapUpdater, resting_order::preimage::RestingOrderPreimage, SlotKey,
    },
    types::Address,
};

impl UpdateMake for Decrease {
    fn update_resting_order<'a, M, B, Q, In, Oc>(
        msg_sender: &Address,
        base_lots: BaseLots,
        key: &SlotKey<RestingOrderPreimage<M, B, Q>>,
        inner_bitmap_updater: &mut InnerBitmapUpdater<'a>,
    ) -> Result<BaseLots, GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher,
        Oc: OccupancyMarker,
    {
        // decrease store, i.e. increase resting order
        Oc::increase_resting_order(msg_sender, base_lots, key, inner_bitmap_updater)
    }
}
