use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        market::{market_marker::MarketMarker, Writables},
        occupancy::Vacant,
        token::token_reader::TokenReader,
        update::{update_marker::UpdateMarker, Increase},
    },
    goblin_error::GoblinError,
    instructions::{open::validate_region::validate_region, MakeReadables},
    matching::region::make_region::MakeRegion,
    state::bitmap::alias::InnerBitmap,
};

pub fn ix_open_inner<M, B, Q, In>(
    region: MakeRegion,
    make_readables: &MakeReadables<M, B, Q>,
    writables: &mut Writables,
    inner_bitmap_state: &mut InnerBitmap,
) -> Result<(), GoblinError>
where
    M: MarketMarker,
    B: TokenReader,
    Q: TokenReader,
    In: LegMatcher,
{
    let position = make_readables.pos_header.position;
    validate_region::<In>(region, position, inner_bitmap_state)?;

    // Update last position if opening in the spread
    if !matches!(region, MakeRegion::In(_)) {
        let last_position = In::get_leg_mut(&mut writables.market_state.last_positions);
        *last_position = position;
    }

    Increase::process_update::<M, B, Q, In, Vacant>(make_readables, writables, inner_bitmap_state)
}
