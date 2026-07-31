use crate::axis::market::{
    market_locator::{hardcoded::HardcodedMarkets, MarketIndex},
    market_marker::MarketMarker,
    token_pair::TokenPair,
    Hardcoded,
};

impl MarketMarker for Hardcoded {
    const DISCRIMINATOR: u8 = 3;

    type MarketLocator<TP>
        = MarketIndex<(Hardcoded, TP)>
    where
        TP: TokenPair,
        MarketIndex<(Hardcoded, TP)>: HardcodedMarkets<TP>;
}
