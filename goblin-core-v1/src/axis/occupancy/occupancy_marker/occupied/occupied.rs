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
        bitmap::alias::{InnerBitmap, InnerBitmapUpdater},
        resting_order::preimage::RestingOrderPreimage,
        SlotKey,
    },
    types::Address,
};

impl OccupancyMarker for Occupied {
    type MakeEnum = UpdateEnum;

    fn get_enums(
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

    fn increase_resting_order<'a, MS: MarketSpec>(
        msg_sender: &Address,
        base_lots: BaseLots,
        key: &SlotKey<RestingOrderPreimage<MS>>,
        _inner_bitmap_updater: &mut InnerBitmapUpdater<'a>,
    ) -> Result<BaseLots, GoblinError> {
        let mut resting_order = key.load();

        require!(
            resting_order.maker == *msg_sender,
            GoblinError::UnauthorizedMsgSender
        );

        let stored_base_lots = &mut resting_order.base_lots;
        *stored_base_lots = stored_base_lots
            .checked_add(base_lots)
            .ok_or(GoblinError::Overflow)?;

        key.store(&resting_order);

        Ok(base_lots)
    }

    fn decrease_resting_order<'a, MS: MarketSpec>(
        msg_sender: &Address,
        base_lots: BaseLots,
        key: &SlotKey<RestingOrderPreimage<MS>>,
        inner_bitmap_updater: &mut InnerBitmapUpdater<'a>,
    ) -> Result<BaseLots, GoblinError> {
        let mut resting_order = key.load();

        require!(
            resting_order.maker == *msg_sender,
            GoblinError::UnauthorizedMsgSender
        );

        let stored_base_lots = &mut resting_order.base_lots;
        let reduced_lots = if *stored_base_lots > base_lots {
            *stored_base_lots -= base_lots;
            key.store(&resting_order);

            base_lots
        } else {
            inner_bitmap_updater.deactivate();

            *stored_base_lots
        };

        Ok(reduced_lots)
    }
}
