use goblin_macros::ConstZero;

use crate::{axis::token::ETH, settlement::global_delta::TokenDelta};

#[derive(Clone, Copy, ConstZero)]
pub struct ETHDelta {
    pub inner: TokenDelta<ETH>,
}
