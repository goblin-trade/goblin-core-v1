use crate::{
    axis::market::{
        market_locator::hardcoded::HardcodedMarketList, market_marker::MarketMarker, MarketLocator,
    },
    axis_helpers::TokenPair,
};

pub trait MarketSpec: Clone + Copy + PartialEq + PartialOrd {
    type Market: MarketMarker + MarketLocator<Self::Pair, Locator = Self::Locator>;
    type Pair: TokenPair + HardcodedMarketList;
    type Locator;
}

impl<M, TP> MarketSpec for (M, TP)
where
    M: MarketMarker + MarketLocator<TP>,
    TP: TokenPair + HardcodedMarketList,
{
    type Market = M;
    type Pair = TP;
    type Locator = <M as MarketLocator<TP>>::Locator;
}
