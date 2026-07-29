use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        market::{market_spec::MarketSpec, Writables},
        occupancy::Vacant,
        update::{update_make::UpdateMake, Decrease},
    },
    goblin_error::GoblinError,
    instructions::{open::validate_region::validate_region, MakeReadables},
    matching::region::make_region::MakeRegion,
    state::bitmap::alias::InnerBitmap,
};

pub fn ix_open_inner<MS: MarketSpec, In: LegMatcher>(
    region: MakeRegion,
    make_readables: &MakeReadables<MS>,
    writables: &mut Writables,
    inner_bitmap_state: &mut InnerBitmap,
) -> Result<(), GoblinError> {
    let position = make_readables.pos_header.position;
    validate_region::<In>(region, position, inner_bitmap_state)?;

    // Update last position if opening in the spread
    if !matches!(region, MakeRegion::In(_)) {
        let last_position = In::get_leg_mut(&mut writables.market_state.last_positions);
        *last_position = position;
    }

    Decrease::process_make::<MS, In, Vacant>(make_readables, writables, inner_bitmap_state)
}
