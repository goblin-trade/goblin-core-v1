use crate::axis::market::{
    market_marker::{hardcoded::hardcoded_market_index::HardcodedMarketIndex, MarketMarker},
    token_pair::TokenPair,
    Hardcoded,
};

impl MarketMarker for Hardcoded {
    const DISCRIMINATOR: u8 = 3;

    type MarketLocator<TP: TokenPair> = HardcodedMarketIndex<TP>;
}
