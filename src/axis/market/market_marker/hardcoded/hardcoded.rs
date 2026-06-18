use crate::axis::{
    market::{
        market_marker::{hardcoded::hardcoded_market_index::HardcodedMarketIndex, MarketMarker},
        Hardcoded,
    },
    token::token_marker::TokenMarker,
};

impl MarketMarker for Hardcoded {
    const DISCRIMINATOR: u8 = 3;

    type ERC20List<'a> = ();

    type MarketLocator<B, Q>
        = HardcodedMarketIndex<B, Q>
    where
        B: TokenMarker,
        Q: TokenMarker;
}
