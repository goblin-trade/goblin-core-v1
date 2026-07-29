use crate::axis::{
    market::{
        market_marker::MarketMarker, market_spec::MarketSpec, token_pair::TokenPair, Dynamic,
        MarketReadables,
    },
    token::token_marker::TokenMarker,
};

impl MarketMarker for Dynamic {
    const DISCRIMINATOR: u8 = 4;

    type MarketLocator<TP: TokenPair> = MarketReadables<(Self, TP)>;
}
