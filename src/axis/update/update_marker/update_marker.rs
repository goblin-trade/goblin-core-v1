use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        market::{market_marker::MarketMarker, Readables},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    instructions::{MakeWritables, PosHeader},
};

pub trait UpdateMarker {
    fn process_update<'a, M, B, Q, In>(
        make_mutables: &mut MakeWritables<'a>,
        readables: &Readables<M, B, Q>,
        pos_header: PosHeader,
    ) -> Result<(), GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher;
}
