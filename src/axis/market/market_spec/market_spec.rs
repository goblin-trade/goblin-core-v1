use crate::axis::{
    market::{market_marker::MarketMarker, token_pair::TokenPair},
    token::token_marker::TokenMarker,
};

pub trait MarketSpec: Clone + Copy {
    type Market: MarketMarker;
    type Pair: TokenPair;
    type Base: TokenMarker;
    type Quote: TokenMarker;
}

impl<M: MarketMarker, TP: TokenPair> MarketSpec for (M, TP) {
    type Market = M;
    type Pair = TP;
    type Base = TP::Base;
    type Quote = TP::Quote;
}

// impl<M, B, Q> MarketSpec for (M, Pair<B, Q>)
// where
//     M: MarketMarker,
//     B: TokenMarker,
//     Q: TokenMarker,
// {
//     type Market = M;
//     type Pair = Pair<B, Q>;
//     type Base = B;
//     type Quote = Q;
// }
