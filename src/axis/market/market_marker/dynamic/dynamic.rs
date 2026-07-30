use crate::axis::market::{
    market_marker::MarketMarker, token_pair::TokenPair, Dynamic, MarketReadables,
};

impl MarketMarker for Dynamic {
    const DISCRIMINATOR: u8 = 4;

    type MarketLocator<TP: TokenPair> = MarketReadables<(Self, TP)>;
}
