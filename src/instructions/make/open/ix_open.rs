use crate::{
    axis::{
        leg::{Base, LegEnum, Quote},
        market::{market_marker::MarketMarker, Readables},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    instructions::{
        open::validate_open_in_spread::validate_open_in_spread, MakeWritables, PosHeader,
    },
    matching::region::make_region::MakeRegion,
    quantities::InnerPos,
    require,
};

impl<'a> MakeWritables<'a> {
    pub fn ix_open<M, B, Q>(
        &mut self,
        readables: &Readables<M, B, Q>,
        pos_header: PosHeader,
        leg_enum: LegEnum,
    ) -> Result<(), GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
    {
        let position = pos_header.position;
        let region = MakeRegion::new(&self.market_state.last_positions, position);

        let inner_pos = InnerPos::from(position);

        self.inner_bitmap_state.activate(inner_pos);

        // leg_in must match or the order must be opened within the spread region.
        if let MakeRegion::In(leg_in) = region {
            require!(leg_in == leg_enum, GoblinError::InvalidOpenPrice);
            require!(
                !self.inner_bitmap_state.index_active(inner_pos),
                GoblinError::PositionOccupied
            );
        } else {
            match leg_enum {
                LegEnum::Base => {
                    validate_open_in_spread::<Base>(&mut self.market_state.last_positions, position)
                }
                LegEnum::Quote => validate_open_in_spread::<Quote>(
                    &mut self.market_state.last_positions,
                    position,
                ),
            }?;
        }
        match leg_enum {
            LegEnum::Base => self.process_open::<M, B, Q, Base>(readables, pos_header),
            LegEnum::Quote => self.process_open::<M, B, Q, Quote>(readables, pos_header),
        }
    }
}
