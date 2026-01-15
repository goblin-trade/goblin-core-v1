use crate::token::{CustomERC20, HardcodedERC20, TokenIndex};

/// Enum type of hardcoded and custom token indices.
///
/// Allows custom markets to use both hardcoded and custom tokens
#[derive(Clone, Copy, PartialEq)]
pub enum DynamicIndex {
    Hardcoded(TokenIndex<HardcodedERC20>),
    Custom(TokenIndex<CustomERC20>),
}

impl DynamicIndex {
    /// Decode a token index byte into either a hardcoded or custom token index.
    /// Token indices are lazily validated when mapping to address.
    ///
    /// Convention:
    /// - If the MSB (bit 7) is 0 → Hardcoded token index (0–127)
    /// - If the MSB (bit 7) is 1 → Custom token index (0–127, but stored as 128–255)
    pub fn new(byte: u8) -> Self {
        const CUSTOM_FLAG: u8 = 0b1000_0000;
        if (byte & CUSTOM_FLAG) == 0 {
            Self::Hardcoded(TokenIndex::new(byte))
        } else {
            let index = byte & !CUSTOM_FLAG; // remove the flag
            Self::Custom(TokenIndex::new(index))
        }
    }
}
