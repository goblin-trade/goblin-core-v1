use crate::{
    axis::{
        market::market_spec::MarketSpec,
        occupancy::occupancy_marker::OccupancyMarker,
        update::{update_sign::UpdateSign, UpdateEnum},
    },
    goblin_error::GoblinError,
    quantities::BaseLots,
    state::{
        bitmap::alias::InnerBitmapUpdater, resting_order::preimage::RestingOrderPreimage, SlotKey,
    },
    types::Address,
};
pub trait UpdateMake: UpdateSign {
    // TODO move on dedicated Generic to Enum mapping trait
    const UPDATE_ENUM: UpdateEnum;

    fn update_resting_order<'a, MS, OM>(
        msg_sender: &Address,
        base_lots: BaseLots,
        key: &SlotKey<RestingOrderPreimage<MS>>,
        inner_bitmap_updater: &mut InnerBitmapUpdater<'a>,
    ) -> Result<BaseLots, GoblinError>
    where
        MS: MarketSpec,
        OM: OccupancyMarker,
    {
        // Direction reversed because of convention.
        // UM refers to increase or decrease in store balance. To increase store balance
        // decrease the resting order and vice versa.
        match Self::UPDATE_ENUM {
            UpdateEnum::Increase => {
                OM::decrease_resting_order(msg_sender, base_lots, key, inner_bitmap_updater)
            }
            UpdateEnum::Decrease => {
                OM::increase_resting_order(msg_sender, base_lots, key, inner_bitmap_updater)
            }
        }
    }
}
