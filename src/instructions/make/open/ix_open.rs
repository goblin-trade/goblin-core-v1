use crate::{
    axis::{
        leg::{Base, LegEnum, Quote},
        market::{market_marker::MarketMarker, Writables},
        token::token_reader::TokenReader,
    },
    goblin_error::GoblinError,
    instructions::{open::ix_open_inner::ix_open_inner, MakeReadables},
    matching::region::make_region::MakeRegion,
    state::bitmap::alias::InnerBitmap,
};

pub fn ix_open<M, B, Q>(
    make_readables: &MakeReadables<M, B, Q>,
    leg_enum: LegEnum,
    writables: &mut Writables,
    inner_bitmap_state: &mut InnerBitmap,
) -> Result<(), GoblinError>
where
    M: MarketMarker,
    B: TokenReader,
    Q: TokenReader,
{
    let position = make_readables.pos_header.position;
    let region = MakeRegion::new(&writables.market_state.last_positions, position);

    match leg_enum {
        LegEnum::Base => {
            ix_open_inner::<M, B, Q, Base>(region, make_readables, writables, inner_bitmap_state)
        }
        LegEnum::Quote => {
            ix_open_inner::<M, B, Q, Quote>(region, make_readables, writables, inner_bitmap_state)
        }
    }
}
