use crate::{
    axis::token::{
        token_marker::hardcoded_erc20::HARDCODED_TOKENS, CustomERC20, HardcodedERC20, Token, ETH,
    },
    settlement::{global_delta_v3::TokenDeltaV3, ConstZero},
    types::Triple,
};

pub const MAX_HARDCODED_DELTAS_V3: usize = HARDCODED_TOKENS.len();
pub const MAX_CUSTOM_DELTAS_V3: usize = 8;

pub type GlobalSender = Triple<
    TokenDeltaV3<ETH>,
    [TokenDeltaV3<HardcodedERC20>; MAX_HARDCODED_DELTAS_V3],
    [TokenDeltaV3<CustomERC20>; MAX_CUSTOM_DELTAS_V3],
    Token,
>;

impl ConstZero for GlobalSender {
    const ZEROED: Self = Self::new(
        TokenDeltaV3::ZEROED,
        [TokenDeltaV3::ZEROED; MAX_HARDCODED_DELTAS_V3],
        [TokenDeltaV3::ZEROED; MAX_CUSTOM_DELTAS_V3],
    );
}
