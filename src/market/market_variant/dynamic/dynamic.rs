use crate::{
    market::{MarketAndKey, MarketVariant},
    token::TokenMarker,
};

#[derive(Clone, Copy, Default)]
pub struct Dynamic;

impl MarketVariant for Dynamic {
    const DISCRIMINATOR: u8 = 4;

    type MarketLocator<B: TokenMarker, Q: TokenMarker> = MarketAndKey<Self, B, Q>;
}
