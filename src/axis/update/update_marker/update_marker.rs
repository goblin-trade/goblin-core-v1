use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        market::{market_marker::MarketMarker, MarketAndKey},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    instructions::{MakeMutables, PosHeader},
    types::Address,
};

pub trait UpdateMarker {
    fn process_update<'a, M, B, Q, In>(
        make_mutables: &mut MakeMutables<'a>,
        msg_sender: &Address,
        market_and_key: &MarketAndKey<M, B, Q>,
        pos_header: PosHeader,
    ) -> Result<(), GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher;
}
