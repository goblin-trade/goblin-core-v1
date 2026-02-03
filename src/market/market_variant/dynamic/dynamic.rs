use crate::{
    market::{MarketAndKey, MarketVariant},
    token::{CustomERC20Data, TokenMarker},
};

#[derive(Clone, Copy, Default)]
pub struct Dynamic;

impl MarketVariant for Dynamic {
    const DISCRIMINATOR: u8 = 4;

    type ERC20List<'a> = &'a [CustomERC20Data];

    type MarketLocator<B, Q>
        = MarketAndKey<Self, B, Q>
    where
        B: TokenMarker,
        Q: TokenMarker;
}
