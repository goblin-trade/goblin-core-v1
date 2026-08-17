use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, LegEnum},
        occupancy::OccupancyEnum,
        update::UpdateEnum,
    },
    axis_helpers::{AxisMarker, MarketSpec},
    goblin_error::GoblinError,
    matching::region::make_region::MakeRegion,
    quantities::Position,
    state::{
        bitmap::alias::InnerBitmap,
        resting_order::{preimage::RestingOrderPreimage, RestingOrder},
        SlotKey,
    },
    types::Address,
};

pub trait OccupancyMarker: AxisMarker<Enum = OccupancyEnum> {
    fn get_make_enums(
        inner_enum_raw: bool,
        region: MakeRegion,
    ) -> Result<(UpdateEnum, LegEnum), GoblinError>;

    fn validate_region<In: LegMatcher>(
        region: MakeRegion,
        position: Position,
        inner_bitmap_state: &InnerBitmap,
    ) -> Result<(), GoblinError>;

    fn get_validated_resting_order<MS: MarketSpec>(
        key: &SlotKey<RestingOrderPreimage<MS>>,
        msg_sender: &Address,
    ) -> Result<RestingOrder, GoblinError>;
}
