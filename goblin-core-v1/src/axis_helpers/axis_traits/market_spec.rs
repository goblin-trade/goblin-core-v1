use crate::{
    axis::market::{
        market_locator::hardcoded::HardcodedMarketList, market_marker::MarketMarker, MarketLocator,
    },
    axis_helpers::TokenPair,
};

pub trait MarketSpecInner: Clone + Copy + PartialEq + PartialOrd {
    type Market: MarketMarker;
    type Pair: TokenPair + HardcodedMarketList;
}

impl<M: MarketMarker, TP: TokenPair + HardcodedMarketList> MarketSpecInner for (M, TP) {
    type Market = M;
    type Pair = TP;
}

pub trait MarketSpec: MarketSpecInner + MarketLocator {}

impl<MS: MarketSpecInner + MarketLocator> MarketSpec for MS {}
