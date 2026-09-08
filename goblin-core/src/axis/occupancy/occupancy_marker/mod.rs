mod occupied;
mod vacant;

use crate::{
    axis::{
        leg::{LegEnum, LegMatcher},
        occupancy::OccupancyEnum,
        update::UpdateEnum,
    },
    axis_helpers::{AxisMarker, TokenPair},
    goblin_error::GoblinError,
    matching::MakeRegion,
    quantities::Position,
    state::{InnerBitmap, RestingOrder, RestingOrderPreimage, SlotKey},
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

    fn get_validated_resting_order<TP: TokenPair>(
        key: &SlotKey<RestingOrderPreimage<TP>>,
        msg_sender: &Address,
    ) -> Result<RestingOrder, GoblinError>;
}
