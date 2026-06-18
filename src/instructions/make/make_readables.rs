use crate::{
    axis::{
        market::{market_marker::MarketMarker, Readables},
        token::token_reader::TokenReader,
    },
    instructions::PosHeader,
};

pub struct MakeReadables<'a, M, B, Q>
where
    M: MarketMarker,
    B: TokenReader,
    Q: TokenReader,
{
    pub readables: &'a Readables<'a, M, B, Q>,
    pub pos_header: PosHeader,
}
