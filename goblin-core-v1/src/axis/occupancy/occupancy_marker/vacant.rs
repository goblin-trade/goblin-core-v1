use crate::{
    axis::{
        market::market_spec::MarketSpec,
        occupancy::{occupancy_marker::OccupancyMarker, Vacant},
    },
    goblin_error::GoblinError,
    quantities::BaseLots,
    settlement::ConstDefault,
    state::{
        bitmap::alias::InnerBitmapUpdater,
        resting_order::{preimage::RestingOrderPreimage, RestingOrder},
        SlotKey,
    },
    types::Address,
};

impl OccupancyMarker for Vacant {
    fn increase_resting_order<'a, MS: MarketSpec>(
        msg_sender: &Address,
        base_lots: BaseLots,
        key: &SlotKey<RestingOrderPreimage<MS>>,
        inner_bitmap_updater: &mut InnerBitmapUpdater<'a>,
    ) -> Result<BaseLots, GoblinError> {
        inner_bitmap_updater.activate();
        key.store(&RestingOrder {
            maker: *msg_sender,
            base_lots,
        });

        Ok(base_lots)
    }

    fn decrease_resting_order<'a, MS: MarketSpec>(
        _msg_sender: &Address,
        _base_lots: BaseLots,
        _key: &SlotKey<RestingOrderPreimage<MS>>,
        _inner_bitmap_updater: &mut InnerBitmapUpdater<'a>,
    ) -> Result<BaseLots, GoblinError> {
        // Unreachable stub
        Ok(BaseLots::DEFAULT)
    }
}
