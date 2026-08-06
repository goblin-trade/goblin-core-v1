use crate::{
    axis::token::{token_list::custom_erc20::CustomERC20Deltas, token_marker::CustomERC20Index},
    settlement::{global_delta::TokenDelta, ConstZero},
};

impl ConstZero for CustomERC20Deltas {
    const ZEROED: Self = Self {
        inner: [TokenDelta::ZEROED; CustomERC20Index::MAX_COUNT],
    };
}
