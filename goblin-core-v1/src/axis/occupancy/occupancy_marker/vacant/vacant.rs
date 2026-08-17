use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, LegEnum},
        occupancy::{
            occupancy_marker::{vacant::validate_region, OccupancyMarker},
            Vacant,
        },
        update::UpdateEnum,
    },
    axis_helpers::MarketSpec,
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

    fn get_validated_resting_order<MS: MarketSpec>(
        _key: &SlotKey<RestingOrderPreimage<MS>>,
        _msg_sender: &Address,
    ) -> Result<RestingOrder, GoblinError> {
        // Vacant postion. Simply return a default empty resting order.
        Ok(RestingOrder::default())
    }
}
