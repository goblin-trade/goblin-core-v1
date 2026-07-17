use crate::{
    axis::token::token_marker::{CustomERC20Deltas, MAX_CUSTOM_ERC20_COUNT},
    settlement::{global_delta::TokenDelta, ConstZero},
};

impl ConstZero for CustomERC20Deltas {
    const ZEROED: Self = Self {
        inner: [TokenDelta::ZEROED; MAX_CUSTOM_ERC20_COUNT],
    };
}
