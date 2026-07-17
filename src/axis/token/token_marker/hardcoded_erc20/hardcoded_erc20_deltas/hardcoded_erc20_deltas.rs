use crate::{
    axis::token::{token_marker::HARDCODED_ERC20_COUNT, HardcodedERC20},
    settlement::global_delta::TokenDelta,
};

#[derive(Clone, Copy)]
pub struct HardcodedERC20Deltas {
    pub inner: [TokenDelta<HardcodedERC20>; HARDCODED_ERC20_COUNT],
}
