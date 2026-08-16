use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, LegEnum, SamePair},
        market::market_spec::MarketSpec,
        occupancy::{occupancy_marker::OccupancyMarker, Occupied},
        update::UpdateEnum,
    },
    goblin_error::GoblinError,
    matching::region::make_region::MakeRegion,
    quantities::{BaseLots, Position},
    require,
    settlement::CheckedOps,
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

    // TODO can we have a common update_resting_order() function
    // * It loads and verifies
    // * Map to UpdateMarker for increase / decrease operations

    fn increase_resting_order<'a, MS: MarketSpec>(
        msg_sender: &Address,
        base_lots: BaseLots,
        key: &SlotKey<RestingOrderPreimage<MS>>,
    ) -> Result<(RestingOrder, BaseLots), GoblinError> {
        let mut resting_order = key.load();

        require!(
            resting_order.maker == *msg_sender,
            GoblinError::UnauthorizedMsgSender
        );

        let stored_base_lots = &mut resting_order.base_lots;
        *stored_base_lots = stored_base_lots
            .checked_add(base_lots)
            .ok_or(GoblinError::Overflow)?;

        Ok((resting_order, base_lots))
    }

    fn decrease_resting_order<'a, MS: MarketSpec>(
        msg_sender: &Address,
        base_lots: BaseLots,
        key: &SlotKey<RestingOrderPreimage<MS>>,
    ) -> Result<(RestingOrder, BaseLots), GoblinError> {
        let mut resting_order = key.load();

        require!(
            resting_order.maker == *msg_sender,
            GoblinError::UnauthorizedMsgSender
        );

        let delta_base_lots = resting_order.base_lots.min(base_lots);
        resting_order.base_lots -= delta_base_lots;

        Ok((resting_order, delta_base_lots))
    }
}
