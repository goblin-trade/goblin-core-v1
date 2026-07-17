use crate::{
    axis::token::token_marker::{HardcodedERC20Deltas, HARDCODED_ERC20_COUNT},
    settlement::{global_delta::TokenDelta, ConstZero},
};

impl ConstZero for HardcodedERC20Deltas {
    const ZEROED: Self = Self {
        inner: [TokenDelta::ZEROED; HARDCODED_ERC20_COUNT],
    };
}
