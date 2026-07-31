use crate::axis::market::{
    market_locator::hardcoded::{HardcodedMarketIndex, HardcodedMarkets},
    market_marker::MarketMarker,
    token_pair::TokenPair,
    Dynamic, MarketReadables,
};

impl MarketMarker for Dynamic {
    const DISCRIMINATOR: u8 = 4;

    type MarketLocator<TP>
        = MarketReadables<(Self, TP)>
    where
        TP: TokenPair,
        HardcodedMarketIndex<TP>: HardcodedMarkets<TP>;
}
