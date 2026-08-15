use crate::{
    axis::{
        market::market_spec::MarketSpec, occupancy::occupancy_marker::OccupancyMarker,
        update::update_sign::UpdateSign,
    },
    goblin_error::GoblinError,
    quantities::BaseLots,
    state::{
        bitmap::alias::InnerBitmapUpdater, resting_order::preimage::RestingOrderPreimage, SlotKey,
    },
    types::Address,
};
pub trait UpdateMake: UpdateSign {
    fn update_resting_order<'a, MS, OM>(
        msg_sender: &Address,
        base_lots: BaseLots,
        key: &SlotKey<RestingOrderPreimage<MS>>,
        inner_bitmap_updater: &mut InnerBitmapUpdater<'a>,
    ) -> Result<BaseLots, GoblinError>
    where
        MS: MarketSpec,
        OM: OccupancyMarker;
}
