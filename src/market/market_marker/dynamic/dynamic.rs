use crate::{
    market::{MarketAndKey, MarketMarker},
    token::{CustomERC20Data, TokenMarker},
    types::Dynamic,
};

impl MarketMarker for Dynamic {
    const DISCRIMINATOR: u8 = 4;

    type ERC20List<'a> = &'a [CustomERC20Data];

    type MarketLocator<B, Q>
        = MarketAndKey<Self, B, Q>
    where
        B: TokenMarker,
        Q: TokenMarker;
}
