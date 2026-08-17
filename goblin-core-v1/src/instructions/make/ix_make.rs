use crate::{
    axis::{
        market::{header::make_header::MakeHeader, Readables, Writables},
        occupancy::occupancy_marker::OccupancyMarker,
    },
    axis_helpers::MarketSpec,
    goblin_error::GoblinError,
    input_processor::{DecodeCtx, FixedDecode},
    instructions::make::ix_make_inner::ix_make_inner,
    match_axes,
    matching::region::make_region::MakeRegion,
    quantities::{Pos2, SafePosition, POS_1},
    state::bitmap::alias::InnerBitmap,
};

pub fn ix_make<MS: MarketSpec>(
    ctx: &DecodeCtx,
    readables: &Readables<MS>,
    pos_1: SafePosition<POS_1>,
    writables: &mut Writables,
    inner_bitmap_state: &mut InnerBitmap,
) -> Result<(), GoblinError> {
    let MakeHeader {
        inner_pos,
        occupancy_enum,
        inner_enum_raw,
        base_lots,
    } = MakeHeader::try_fixed_decode(ctx)?;

    let pos_2 = Pos2::new(pos_1, inner_pos);
    let position = pos_2.into();

    let region = MakeRegion::new(&writables.market_state.last_positions, position);

    match_axes!(OM = occupancy_enum => {
        let enums = OM::get_make_enums(inner_enum_raw, region)?;

        match_axes!(UM = enums.0, In = enums.1 => {
            // TODO combine Readables and Writables
            // TODO combine base_lots, position, region into common struct
            ix_make_inner::<MS, (OM, UM), In>(base_lots, position, region, readables, writables, inner_bitmap_state)?;
        });
    });

    Ok(())
}
