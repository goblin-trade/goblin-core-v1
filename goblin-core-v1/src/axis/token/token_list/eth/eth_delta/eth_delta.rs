use crate::{axis::token::ETH, settlement::global_delta::TokenDelta};

#[derive(Clone, Copy)]
pub struct ETHDelta {
    pub inner: TokenDelta<ETH>,
}
