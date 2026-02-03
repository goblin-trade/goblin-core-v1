use crate::{
    market::{HardcodedMarketIndex, MarketVariant},
    token::TokenMarker,
};

#[derive(Clone, Copy, Default)]
pub struct Hardcoded;

impl MarketVariant for Hardcoded {
    const DISCRIMINATOR: u8 = 3;

    type ERC20List<'a> = ();

    type MarketLocator<B, Q>
        = HardcodedMarketIndex<B, Q>
    where
        B: TokenMarker,
        Q: TokenMarker;
}
