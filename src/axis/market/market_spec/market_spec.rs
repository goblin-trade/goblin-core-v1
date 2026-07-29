use crate::axis::{
    leg::Pair,
    market::{market_marker::MarketMarker, token_pair::TokenPair},
    token::token_marker::TokenMarker,
};

pub trait MarketSpec {
    type Market: MarketMarker;
    type Pair: TokenPair;
    type Base: TokenMarker;
    type Quote: TokenMarker;
}

impl<M, B, Q> MarketSpec for (M, Pair<B, Q>)
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    type Market = M;
    type Pair = Pair<B, Q>;
    type Base = B;
    type Quote = Q;
}
