use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, LegEnum, SamePair},
        market::{market_spec::MarketSpec, Writables},
        occupancy::{occupancy_marker::OccupancyMarker, Occupied},
        update::{update_make::UpdateMake, UpdateEnum, UpdateMarker},
    },
    goblin_error::GoblinError,
    instructions::MakeReadables,
    match_axes,
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

    // ix_update()
    //
    // This function needs LegEnum too. But instead of being decoded, it is derived
    // from position and region
    fn make<MS: MarketSpec>(
        make_readables: &MakeReadables<MS>,
        inner_enum_raw: bool,
        writables: &mut Writables,
        inner_bitmap_state: &mut InnerBitmap,
    ) -> Result<(), GoblinError> {
        let position = make_readables.pos_header.position;
        let region = MakeRegion::new(&writables.market_state.last_positions, position);
        let update_enum = UpdateEnum::from(inner_enum_raw);

        // Cannot update in `Spread` region as it has no orders
        let MakeRegion::In(leg_enum) = region else {
            return Err(GoblinError::NoRestingOrder);
        };

        require!(
            inner_bitmap_state.index_active(position.into()),
            GoblinError::NoRestingOrder
        );

        match_axes!(In = leg_enum, UM = update_enum => {
            UM::process_make::<MS, In, Self>(
                make_readables,
                writables,
                inner_bitmap_state,
            )?;
        });

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
