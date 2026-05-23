use crate::{
    axis::{
        leg::{Base, LegEnum, Quote},
        market::{market_marker::MarketMarker, Writables},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    instructions::{
        open::{process_open::process_open, validate_open_in_spread::validate_open_in_spread},
        MakeReadables,
    },
    matching::region::make_region::MakeRegion,
    quantities::InnerPos,
    require,
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
    B: TokenMarker,
    Q: TokenMarker,
{
    let position = make_readables.pos_header.position;
    let region = MakeRegion::new(&writables.market_state.last_positions, position);

    let inner_pos = InnerPos::from(position);

    inner_bitmap_state.activate(inner_pos);

    // leg_in must match or the order must be opened within the spread region.
    if let MakeRegion::In(leg_in) = region {
        require!(leg_in == leg_enum, GoblinError::InvalidOpenPrice);
        require!(
            !inner_bitmap_state.index_active(inner_pos),
            GoblinError::PositionOccupied
        );
    } else {
        match leg_enum {
            LegEnum::Base => validate_open_in_spread::<Base>(
                &mut writables.market_state.last_positions,
                position,
            ),
            LegEnum::Quote => validate_open_in_spread::<Quote>(
                &mut writables.market_state.last_positions,
                position,
            ),
        }?;
    }
    match leg_enum {
        LegEnum::Base => process_open::<M, B, Q, Base>(make_readables, writables),
        LegEnum::Quote => process_open::<M, B, Q, Quote>(make_readables, writables),
    }
}
