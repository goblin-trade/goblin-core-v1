use goblin_macros::ConstDefault;

use crate::{axis::token::ETH, settlement::global_delta::TokenDelta};

#[derive(Clone, Copy, ConstDefault)]
pub struct ETHDelta {
    pub inner: TokenDelta<ETH>,
}
