use crate::{
    axis::{
        leg::{Base, LegEnum, Quote},
        market::{market_marker::MarketMarker, Readables},
        token::token_marker::TokenMarker,
        update::{update_marker::UpdateMarker, Decrease, Increase, UpdateEnum},
    },
    goblin_error::GoblinError,
    instructions::{MakeMutables, PosHeader},
};

impl<'a> MakeMutables<'a> {
    pub(super) fn process_update_cases<M, B, Q>(
        &mut self,
        readables: &Readables<M, B, Q>,
        pos_header: PosHeader,
        update_enum: UpdateEnum,
        leg_in: LegEnum,
    ) -> Result<(), GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
    {
        match (leg_in, update_enum) {
            (LegEnum::Base, UpdateEnum::Increase) => {
                Increase::process_update::<M, B, Q, Base>(self, readables, pos_header)
            }
            (LegEnum::Quote, UpdateEnum::Increase) => {
                Increase::process_update::<M, B, Q, Quote>(self, readables, pos_header)
            }
            (LegEnum::Base, UpdateEnum::Decrease) => {
                Decrease::process_update::<M, B, Q, Base>(self, readables, pos_header)
            }
            (LegEnum::Quote, UpdateEnum::Decrease) => {
                Decrease::process_update::<M, B, Q, Quote>(self, readables, pos_header)
            }
        }
    }
}
