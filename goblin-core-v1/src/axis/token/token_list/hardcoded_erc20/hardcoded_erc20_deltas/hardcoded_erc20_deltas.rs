use goblin_macros::ConstZero;

use crate::{
    axis::token::{token_list::hardcoded_erc20::HARDCODED_ERC20_COUNT, HardcodedERC20},
    settlement::global_delta::TokenDelta,
};

#[derive(Clone, Copy, ConstZero)]
pub struct HardcodedERC20Deltas {
    pub inner: [TokenDelta<HardcodedERC20>; HARDCODED_ERC20_COUNT],
}
