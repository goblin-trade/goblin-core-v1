mod impl_index;
mod impl_into_iterator;

use goblin_macros::ConstDefault;

use crate::{
    axis::token::{CustomERC20, token_marker::CustomERC20Index},
    settlement::global_delta::TokenDelta,
};

#[derive(Clone, Copy, ConstDefault)]
pub struct CustomERC20Deltas {
    pub inner: [TokenDelta<CustomERC20>; CustomERC20Index::MAX_COUNT],
}
