use crate::axis::market::{
    market_locator::{hardcoded::HardcodedMarkets, MarketIndex},
    market_marker::MarketMarker,
    token_pair::TokenPair,
    Dynamic, Hardcoded, MarketReadables,
};

impl MarketMarker for Dynamic {
    const DISCRIMINATOR: u8 = 4;

    // type MarketLocator<TP>
    //     = MarketReadables<(Self, TP)>
    // where
    //     TP: TokenPair,
    //     MarketIndex<(Hardcoded, TP)>: HardcodedMarkets<TP>;
}
