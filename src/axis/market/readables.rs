use crate::{
    axis::{
        market::{market_marker::MarketMarker, MarketReadables},
        token::token_marker::TokenMarker,
    },
    types::Address,
};

pub struct Readables<'a, M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    pub msg_sender: &'a Address,
    pub market_readables: &'a MarketReadables<M, B, Q>,
}
