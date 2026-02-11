use crate::axis::{
    market::{market_marker::MarketMarker, Dynamic, MarketAndKey},
    token::token_marker::{custom_erc20::custom_erc20_data::CustomERC20Data, TokenMarker},
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
