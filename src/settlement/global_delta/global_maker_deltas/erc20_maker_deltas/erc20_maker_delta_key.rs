use crate::{
    token::{ERC20Marker, TokenIndex},
    types::Address,
};

/// Key to index Maker delta for a given token index
#[derive(Clone, Copy, PartialEq)]
pub struct ERC20MakerDeltaKey<T: ERC20Marker> {
    pub maker: Address,
    pub token_index: TokenIndex<T>,
}

impl<T: ERC20Marker> ERC20MakerDeltaKey<T> {
    pub const fn zero() -> Self {
        Self {
            maker: [0u8; 20],
            token_index: TokenIndex::new(0),
        }
    }
}
