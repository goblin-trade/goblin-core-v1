use crate::{axis::market::market_marker::MarketMarker, market::TokenPair};

pub trait MarketSpec: Clone + Copy + PartialEq + PartialOrd {
    type Market: MarketMarker;
    type Pair: TokenPair;
}

impl<M: MarketMarker, TP: TokenPair> MarketSpec for (M, TP) {
    type Market = M;
    type Pair = TP;
}
