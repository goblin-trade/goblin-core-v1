use goblin_macros::ConstDefault;

use crate::{
    axis::token::{token_list::hardcoded_erc20::HARDCODED_ERC20_COUNT, HardcodedERC20},
    settlement::global_delta::TokenDelta,
};

#[derive(Clone, Copy, ConstDefault)]
pub struct HardcodedERC20Deltas {
    pub inner: [TokenDelta<HardcodedERC20>; HARDCODED_ERC20_COUNT],
}
