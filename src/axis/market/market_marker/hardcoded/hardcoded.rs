use crate::axis::market::{
    market_locator::hardcoded::{HardcodedMarketIndex, HardcodedMarkets},
    market_marker::MarketMarker,
    token_pair::TokenPair,
    Hardcoded,
};

impl MarketMarker for Hardcoded {
    const DISCRIMINATOR: u8 = 3;

    type MarketLocator<TP>
        = HardcodedMarketIndex<TP>
    where
        TP: TokenPair,
        HardcodedMarketIndex<TP>: HardcodedMarkets<TP>;
}
