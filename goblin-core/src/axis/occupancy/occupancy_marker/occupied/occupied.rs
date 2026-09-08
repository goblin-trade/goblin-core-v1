use crate::{
    axis::{
        leg::{LegEnum, LegMatcher},
        occupancy::{OccupancyMarker, Occupied},
        update::UpdateEnum,
    },
    axis_helpers::TokenPair,
    goblin_error::GoblinError,
    matching::MakeRegion,
    quantities::Position,
    require,
    state::{InnerBitmap, RestingOrder, RestingOrderPreimage, SlotKey},
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

    fn validate_region<In: LegMatcher>(
        _region: MakeRegion,
        position: Position,
        inner_bitmap_state: &InnerBitmap,
    ) -> Result<(), GoblinError> {
        require!(
            inner_bitmap_state.index_active(position.into()),
            GoblinError::NoRestingOrder
        );
        Ok(())
    }

    fn get_validated_resting_order<TP: TokenPair>(
        key: &SlotKey<RestingOrderPreimage<TP>>,
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
