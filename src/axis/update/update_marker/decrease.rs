use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        market::market_marker::MarketMarker,
        occupancy::occupancy_marker::OccupancyMarker,
        token::token_marker::TokenMarker,
        update::{update_marker::UpdateMarker, Decrease},
    },
    goblin_error::GoblinError,
    quantities::{BaseLots, DeltaLots, UnsidedLots},
    state::{
        bitmap::alias::InnerBitmapUpdater, resting_order::preimage::RestingOrderPreimage, SlotKey,
    },
    types::Address,
};

impl UpdateMarker for Decrease {
    fn delta_lots(lots: UnsidedLots) -> Result<DeltaLots, GoblinError> {
        // Positive delta for decrease. Decreasing frees up lots.
        DeltaLots::try_from(lots)
    }

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
        Oc::decrease(msg_sender, base_lots, key, inner_bitmap_updater)
    }
}
