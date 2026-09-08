mod impl_index;
mod impl_into_iterator;

use goblin_macros::ConstDefault;

use crate::{
    axis::token::{HardcodedERC20, token_list::hardcoded_erc20::HARDCODED_ERC20_COUNT},
    settlement::global_delta::TokenDelta,
};

#[derive(Clone, Copy, ConstDefault)]
pub struct HardcodedERC20Deltas {
    pub inner: [TokenDelta<HardcodedERC20>; HARDCODED_ERC20_COUNT],
}
