use crate::axis::{
    market::{market_marker::MarketMarker, Dynamic, MarketReadables},
    token::token_marker::{custom_erc20::custom_erc20_list::CustomERC20List, TokenMarker},
};

impl MarketMarker for Dynamic {
    const DISCRIMINATOR: u8 = 4;

    type ERC20List<'a> = CustomERC20List<'a>;

    type MarketLocator<B, Q>
        = MarketReadables<Self, B, Q>
    where
        B: TokenMarker,
        Q: TokenMarker;
}
