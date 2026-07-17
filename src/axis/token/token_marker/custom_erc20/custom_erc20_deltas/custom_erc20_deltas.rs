use crate::{
    axis::token::{token_marker::MAX_CUSTOM_ERC20_COUNT, CustomERC20},
    settlement::global_delta::TokenDelta,
};

#[derive(Clone, Copy)]
pub struct CustomERC20Deltas {
    pub inner: [TokenDelta<CustomERC20>; MAX_CUSTOM_ERC20_COUNT],
}
