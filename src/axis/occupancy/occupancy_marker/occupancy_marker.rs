use crate::{
    axis::market::market_spec::MarketSpec,
    goblin_error::GoblinError,
    quantities::BaseLots,
    state::{
        bitmap::alias::InnerBitmapUpdater, resting_order::preimage::RestingOrderPreimage, SlotKey,
    },
    types::Address,
};

pub trait OccupancyMarker {
    fn increase_resting_order<'a, MS: MarketSpec>(
        msg_sender: &Address,
        base_lots: BaseLots,
        key: &SlotKey<RestingOrderPreimage<MS>>,
        inner_bitmap_updater: &mut InnerBitmapUpdater<'a>,
    ) -> Result<BaseLots, GoblinError>;

    fn decrease_resting_order<'a, MS: MarketSpec>(
        msg_sender: &Address,
        base_lots: BaseLots,
        key: &SlotKey<RestingOrderPreimage<MS>>,
        inner_bitmap_updater: &mut InnerBitmapUpdater<'a>,
    ) -> Result<BaseLots, GoblinError>;
}
