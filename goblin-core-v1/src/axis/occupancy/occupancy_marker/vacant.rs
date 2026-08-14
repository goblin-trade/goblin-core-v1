use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, LegEnum, SamePair},
        market::{market_spec::MarketSpec, Writables},
        occupancy::{occupancy_marker::OccupancyMarker, Vacant},
        update::{update_make::UpdateMake, Decrease, UpdateEnum},
    },
    goblin_error::GoblinError,
    instructions::{open::validate_region::validate_region, MakeReadables},
    match_axes,
    matching::region::make_region::MakeRegion,
    quantities::{BaseLots, Position},
    settlement::ConstDefault,
    state::{
        bitmap::alias::{InnerBitmap, InnerBitmapUpdater},
        resting_order::{preimage::RestingOrderPreimage, RestingOrder},
        SlotKey,
    },
    types::{Address, StoreReader},
};

impl OccupancyMarker for Vacant {
    type MakeEnum = LegEnum;

    fn get_enums(
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

    // ix_open
    fn make<MS: MarketSpec>(
        make_readables: &MakeReadables<MS>,
        inner_enum_raw: bool,
        writables: &mut Writables,
        inner_bitmap_state: &mut InnerBitmap,
    ) -> Result<(), GoblinError> {
        let position = make_readables.pos_header.position;
        let region = MakeRegion::new(&writables.market_state.last_positions, position);

        let leg_enum = LegEnum::from(inner_enum_raw);
        match_axes!(In = leg_enum => {
            validate_region::<In>(region, position, inner_bitmap_state)?;

            // Update last position if opening in the spread
            if !matches!(region, MakeRegion::In(_)) {
                let last_position = In::get_leg_mut(&mut writables.market_state.last_positions);
                *last_position = position;
            }

            // TODO can use match_axes!() and unify the occupied and vacant impls?
            //
            // Spagetti code- we call function on UpdateMarker then come back to OccupancyMarker,
            // where Vacant::decrease_resting_order() is a stub
            Decrease::process_make::<MS, In, Self>(make_readables, writables, inner_bitmap_state)?;
        });
        Ok(())
    }

    fn increase_resting_order<'a, MS: MarketSpec>(
        msg_sender: &Address,
        base_lots: BaseLots,
        key: &SlotKey<RestingOrderPreimage<MS>>,
        inner_bitmap_updater: &mut InnerBitmapUpdater<'a>,
    ) -> Result<BaseLots, GoblinError> {
        inner_bitmap_updater.activate();
        key.store(&RestingOrder {
            maker: *msg_sender,
            base_lots,
        });

        Ok(base_lots)
    }

    fn decrease_resting_order<'a, MS: MarketSpec>(
        _msg_sender: &Address,
        _base_lots: BaseLots,
        _key: &SlotKey<RestingOrderPreimage<MS>>,
        _inner_bitmap_updater: &mut InnerBitmapUpdater<'a>,
    ) -> Result<BaseLots, GoblinError> {
        // Unreachable stub
        Ok(BaseLots::DEFAULT)
    }
}
