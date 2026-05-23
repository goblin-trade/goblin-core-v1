use crate::{
    axis::{
        market::{market_marker::MarketMarker, Readables},
        token::token_marker::TokenMarker,
    },
    instructions::PosHeader,
};

pub struct MakeReadables<'a, M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    pub readables: &'a Readables<'a, M, B, Q>,
    pub pos_header: PosHeader,
}
