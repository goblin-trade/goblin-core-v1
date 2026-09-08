use crate::{
    axis::{
        leg::{LegEnum, LegMatcher},
        occupancy::{
            occupancy_marker::vacant::validate_region, OccupancyMarker, Vacant,
        },
        update::UpdateEnum,
    },
    axis_helpers::TokenPair,
    goblin_error::GoblinError,
    matching::MakeRegion,
    quantities::Position,
    state::{InnerBitmap, RestingOrder, RestingOrderPreimage, SlotKey},
    types::Address,
};

impl OccupancyMarker for Vacant {
    fn get_make_enums(
        inner_enum_raw: bool,
        _region: MakeRegion,
    ) -> Result<(UpdateEnum, LegEnum), GoblinError> {
        let leg_enum = LegEnum::from(inner_enum_raw);
        Ok((UpdateEnum::Decrease, leg_enum))
    }

    fn validate_region<In: LegMatcher>(
        region: MakeRegion,
        position: Position,
        inner_bitmap_state: &InnerBitmap,
    ) -> Result<(), GoblinError> {
        validate_region::<In>(region, position, inner_bitmap_state)?;
        Ok(())
    }

    fn get_validated_resting_order<TP: TokenPair>(
        _key: &SlotKey<RestingOrderPreimage<TP>>,
        _msg_sender: &Address,
    ) -> Result<RestingOrder, GoblinError> {
        // Vacant postion. Simply return a default empty resting order.
        Ok(RestingOrder::default())
    }
}
