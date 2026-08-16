use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, LegEnum, SamePair},
        market::market_spec::MarketSpec,
        occupancy::{
            occupancy_marker::{vacant::validate_region, OccupancyMarker},
            Vacant,
        },
        update::UpdateEnum,
    },
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

    fn validate_and_update_region<In: LegMatcher>(
        region: MakeRegion,
        position: Position,
        last_positions: &mut SamePair<Position>,
        inner_bitmap_state: &mut InnerBitmap,
    ) -> Result<(), GoblinError> {
        validate_region::<In>(region, position, inner_bitmap_state)?;

        // Update last position if opening in the spread
        if !matches!(region, MakeRegion::In(_)) {
            let last_position = In::get_leg_mut(last_positions);
            *last_position = position;
        }
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
