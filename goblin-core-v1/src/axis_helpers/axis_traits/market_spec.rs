use crate::{
    axis::market::{market_marker::MarketMarker, MarketLocator},
    market::TokenPair,
};

pub trait MarketSpecInner: Clone + Copy + PartialEq + PartialOrd {
    type Market: MarketMarker;
    type Pair: TokenPair;
}

impl<M: MarketMarker, TP: TokenPair> MarketSpecInner for (M, TP) {
    type Market = M;
    type Pair = TP;
}

pub trait MarketSpec: MarketSpecInner + MarketLocator {}

impl<MS: MarketSpecInner + MarketLocator> MarketSpec for MS {}
