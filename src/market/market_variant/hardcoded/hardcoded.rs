use crate::{
    market::{DangerousMarketIndex, MarketVariant},
    token::TokenMarker,
};

#[derive(Clone, Copy, Default)]
pub struct Hardcoded;

impl MarketVariant for Hardcoded {
    const DISCRIMINATOR: u8 = 3;

    type MarketLocator<B, Q>
        = DangerousMarketIndex<B, Q>
    where
        B: TokenMarker,
        Q: TokenMarker;
}
