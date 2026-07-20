use crate::axis::{
    market::{market_marker::MarketMarker, Dynamic, MarketReadables},
    token::token_marker::TokenMarker,
};

impl MarketMarker for Dynamic {
    const DISCRIMINATOR: u8 = 4;

    type MarketLocator<B, Q>
        = MarketReadables<Self, B, Q>
    where
        B: TokenMarker,
        Q: TokenMarker;
}
