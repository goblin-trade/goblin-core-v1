use crate::{
    axis::token::{token_marker::CustomERC20Index, CustomERC20},
    settlement::global_delta::TokenDelta,
};

#[derive(Clone, Copy)]
pub struct CustomERC20Deltas {
    pub inner: [TokenDelta<CustomERC20>; CustomERC20Index::MAX_COUNT],
}
