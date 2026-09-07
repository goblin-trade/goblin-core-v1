use crate::{
    axis::{LegMatcher, OccupancyMarker, UpdateMarker},
    axis_helpers::MarketSpec,
    goblin_error::GoblinError,
    instructions::ix_make_inner::{
        update_delta::update_delta, update_last_position::update_last_position,
        update_matrix::update_matrix, update_resting_order::update_resting_order,
    },
    matching::MakeRegion,
    quantities::{BaseLots, Position},
    state::InnerBitmap,
    Ctx,
};

pub fn ix_make_inner<MS, OM, UM, In>(
    base_lots: BaseLots,
    position: Position,
    region: MakeRegion,
    inner_bitmap_state: &mut InnerBitmap,
    ctx: &mut Ctx<MS>,
) -> Result<(), GoblinError>
where
    MS: MarketSpec,
    OM: OccupancyMarker,
    UM: UpdateMarker,
    In: LegMatcher,
{
    OM::validate_region::<In>(region, position, inner_bitmap_state)?;

    let (delta_base_lots, resting_order_empty) =
        update_resting_order::<MS, OM, UM>(base_lots, position, ctx)?;

    update_matrix::<MS, OM>(resting_order_empty, position, inner_bitmap_state);
    update_last_position::<MS, OM, UM, In>(position, region, ctx);

    // pass opposite leg to update_delta. By convention `In` is the leg
    // from perspective of the taker so we need to reverse it for makers.
    update_delta::<MS, UM, In::Opposite>(delta_base_lots, position, ctx)?;

    Ok(())
}
