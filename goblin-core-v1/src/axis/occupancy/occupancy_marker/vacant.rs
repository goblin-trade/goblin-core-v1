use crate::{
    axis::{
        leg::LegEnum,
        market::{market_spec::MarketSpec, Writables},
        occupancy::{occupancy_marker::OccupancyMarker, Vacant},
    },
    goblin_error::GoblinError,
    instructions::MakeReadables,
    quantities::BaseLots,
    settlement::ConstDefault,
    state::{
        bitmap::alias::{InnerBitmap, InnerBitmapUpdater},
        resting_order::{preimage::RestingOrderPreimage, RestingOrder},
        SlotKey,
    },
    types::Address,
};

impl OccupancyMarker for Vacant {
    type MakeEnum = LegEnum;

    fn make<MS: MarketSpec>(
        make_readables: &MakeReadables<MS>,
        inner_enum_raw: bool,
        writables: &mut Writables,
        inner_bitmap_state: &mut InnerBitmap,
    ) -> Result<(), GoblinError> {
        Ok(())
    }

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
