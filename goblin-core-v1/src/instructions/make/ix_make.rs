use crate::{
    axis::occupancy::occupancy_marker::OccupancyMarker,
    axis_helpers::MarketSpec,
    goblin_error::GoblinError,
    input_processor::{ArgsReader, FixedDecode},
    instructions::make::{
        update_delta::update_delta, update_matrix::update_matrix,
        update_resting_order::update_resting_order,
    },
    market::MakeHeader,
    match_axes,
    matching::region::make_region::MakeRegion,
    quantities::{BaseLots, Pos2, SafePosition, POS_1},
    state::bitmap::alias::InnerBitmap,
    Ctx,
};

pub fn ix_make<MS: MarketSpec>(
    pos_1: SafePosition<POS_1>,
    reader: &ArgsReader,
    inner_bitmap_state: &mut InnerBitmap,
    ctx: &mut Ctx<MS>,
) -> Result<(), GoblinError> {
    let MakeHeader {
        inner_pos,
        occupancy_enum,
        inner_enum_raw,
        base_lots,
    } = MakeHeader::try_fixed_decode(reader)?;

    if base_lots == BaseLots::default() {
        return Ok(());
    }

    let position = Pos2::new(pos_1, inner_pos).into();
    let region = MakeRegion::new(&ctx.writables.market_state.last_positions, position);

    match_axes!(OM = occupancy_enum => {
        let enums = OM::get_make_enums(inner_enum_raw, region)?;

        match_axes!(UM = enums.0, In = enums.1 => {
            OM::validate_region::<In>(region, position, inner_bitmap_state)?;

            let (delta_base_lots, resting_order_empty) =
                update_resting_order::<MS, OM, UM>(base_lots, position, ctx)?;

            update_matrix::<MS, OM, In>(resting_order_empty, position, region, inner_bitmap_state, ctx);
            update_delta::<MS, UM, In>(delta_base_lots, position, ctx)?;
        });
    });

    Ok(())
}
