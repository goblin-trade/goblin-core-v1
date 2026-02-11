use super::HardcodedMarketIndex;
use crate::{market::market_marker::MarketMarker, token::TokenMarker, types::Hardcoded};

impl MarketMarker for Hardcoded {
    const DISCRIMINATOR: u8 = 3;

    type ERC20List<'a> = ();

    type MarketLocator<B, Q>
        = HardcodedMarketIndex<B, Q>
    where
        B: TokenMarker,
        Q: TokenMarker;
}
