use crate::{
    token::{ERC20Marker, ERC20Index},
    types::Address,
};

/// Key to index Maker delta for a given token index
#[derive(Clone, Copy, PartialEq)]
pub struct ERC20MakerDeltaKey<T: ERC20Marker> {
    pub maker: Address,
    pub token_index: ERC20Index<T>,
}

impl<T: ERC20Marker> ERC20MakerDeltaKey<T> {
    pub const fn zero() -> Self {
        Self {
            maker: [0u8; 20],
            token_index: ERC20Index::new(0),
        }
    }
}
