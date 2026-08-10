use crate::{
    axis::{
        leg::LegEnum,
        market::{market_spec::MarketSpec, Writables},
    },
    goblin_error::GoblinError,
    instructions::{open::ix_open_inner::ix_open_inner, MakeReadables},
    match_axes,
    matching::region::make_region::MakeRegion,
    state::bitmap::alias::InnerBitmap,
};

pub fn ix_open<MS: MarketSpec>(
    make_readables: &MakeReadables<MS>,
    leg_enum: LegEnum,
    writables: &mut Writables,
    inner_bitmap_state: &mut InnerBitmap,
) -> Result<(), GoblinError> {
    let position = make_readables.pos_header.position;
    let region = MakeRegion::new(&writables.market_state.last_positions, position);

    match_axes!(In = leg_enum => {
        ix_open_inner::<MS, In>(region, make_readables, writables, inner_bitmap_state)?;
    });
    Ok(())
}
