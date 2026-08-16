use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, LegEnum, SamePair},
        market::market_spec::MarketSpec,
        occupancy::{occupancy_marker::OccupancyMarker, Occupied},
        update::UpdateEnum,
    },
    goblin_error::GoblinError,
    matching::region::make_region::MakeRegion,
    quantities::Position,
    require,
    state::{
        bitmap::alias::InnerBitmap,
        resting_order::{preimage::RestingOrderPreimage, RestingOrder},
        SlotKey,
    },
    types::Address,
};

impl OccupancyMarker for Occupied {
    fn get_make_enums(
        inner_enum_raw: bool,
        region: MakeRegion,
    ) -> Result<(UpdateEnum, LegEnum), GoblinError> {
        let update_enum = UpdateEnum::from(inner_enum_raw);

        let MakeRegion::In(leg_enum) = region else {
            return Err(GoblinError::NoRestingOrder);
        };

        Ok((update_enum, leg_enum))
    }

    fn validate_and_update_region<In: LegMatcher>(
        _region: MakeRegion,
        position: Position,
        _last_positions: &mut SamePair<Position>,
        inner_bitmap_state: &mut InnerBitmap,
    ) -> Result<(), GoblinError> {
        require!(
            inner_bitmap_state.index_active(position.into()),
            GoblinError::NoRestingOrder
        );
        Ok(())
    }

    fn get_validated_resting_order<MS: MarketSpec>(
        key: &SlotKey<RestingOrderPreimage<MS>>,
        msg_sender: &Address,
    ) -> Result<RestingOrder, GoblinError> {
        // Load from key and ensure owner matches
        let resting_order = key.load();
        require!(
            resting_order.maker == *msg_sender,
            GoblinError::UnauthorizedMsgSender
        );

        Ok(resting_order)
    }
}
