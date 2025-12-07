use crate::{
    token::{ERC20Token, TokenIndex},
    types::Address,
};

/// Key to index Maker delta for a given token index
#[derive(Clone, Copy, PartialEq)]
pub struct ERC20MakerDeltaKey<T: ERC20Token> {
    pub maker: Address,
    pub token_index: TokenIndex<T>,
}

impl<T: ERC20Token> ERC20MakerDeltaKey<T> {
    pub const fn new() -> Self {
        Self {
            maker: [0u8; 20],
            token_index: TokenIndex::new(0),
        }
    }
}
