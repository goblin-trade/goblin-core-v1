use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, LegEnum, SamePair},
        market::market_spec::MarketSpec,
        occupancy::OccupancyEnum,
        update::UpdateEnum,
        AxisMarker,
    },
    goblin_error::GoblinError,
    matching::region::make_region::MakeRegion,
    quantities::{BaseLots, Position},
    state::{
        bitmap::alias::{InnerBitmap, InnerBitmapUpdater},
        resting_order::preimage::RestingOrderPreimage,
        SlotKey,
    },
    types::Address,
};

pub trait OccupancyMarker: AxisMarker<Enum = OccupancyEnum> {
    fn get_make_enums(
        inner_enum_raw: bool,
        region: MakeRegion,
    ) -> Result<(UpdateEnum, LegEnum), GoblinError>;

    fn validate_and_update_region<In: LegMatcher>(
        region: MakeRegion,
        position: Position,
        last_positions: &mut SamePair<Position>,
        inner_bitmap_state: &mut InnerBitmap,
    ) -> Result<(), GoblinError>;

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
