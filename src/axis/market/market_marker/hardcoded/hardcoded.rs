use crate::axis::market::{
    market_locator::hardcoded::HardcodedMarketIndex, market_marker::MarketMarker,
    token_pair::TokenPair, Hardcoded,
};

impl MarketMarker for Hardcoded {
    const DISCRIMINATOR: u8 = 3;

    type MarketLocator<TP: TokenPair> = HardcodedMarketIndex<TP>;
}
