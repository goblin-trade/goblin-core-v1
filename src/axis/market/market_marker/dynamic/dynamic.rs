use crate::axis::{
    market::{market_marker::MarketMarker, Dynamic, MarketReadables},
    token::token_reader::{custom_erc20::custom_erc20_data::CustomERC20Data, TokenReader},
};

impl MarketMarker for Dynamic {
    const DISCRIMINATOR: u8 = 4;

    type ERC20List<'a> = &'a [CustomERC20Data];

    type MarketLocator<B, Q>
        = MarketReadables<Self, B, Q>
    where
        B: TokenReader,
        Q: TokenReader;
}
