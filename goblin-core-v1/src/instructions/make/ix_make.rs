use crate::{
    axis::occupancy::occupancy_marker::OccupancyMarker,
    axis_helpers::MarketSpec,
    goblin_error::GoblinError,
    input_processor::{ArgsReader, FixedDecode},
    instructions::make::{ix_make_delta::ix_make_delta, ix_make_states::ix_make_states},
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
        // Opening with 0 size is no-op
        return Ok(());
    }

    let pos_2 = Pos2::new(pos_1, inner_pos);
    let position = pos_2.into();

    let region = MakeRegion::new(&ctx.writables.market_state.last_positions, position);

    match_axes!(OM = occupancy_enum => {
        let enums = OM::get_make_enums(inner_enum_raw, region)?;

        match_axes!(UM = enums.0, In = enums.1 => {
            // TODO combine base_lots, position, region into common struct

            // 1. Update states
            let delta_base_lots =
                ix_make_states::<MS, (OM, UM), In>(base_lots, position, region, inner_bitmap_state, ctx)?;

            // 2. Update delta
            ix_make_delta::<MS, UM, In>(delta_base_lots, position, ctx)?;
        });
    });

    Ok(())
}
