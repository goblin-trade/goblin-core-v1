use crate::token::{CustomERC20, ERC20Index, HardcodedERC20};

/// Enum type of hardcoded and custom token indices.
///
/// Allows custom markets to use both hardcoded and custom tokens
#[derive(Clone, Copy, PartialEq)]
pub enum DynamicIndex {
    Hardcoded(ERC20Index<HardcodedERC20>),
    Custom(ERC20Index<CustomERC20>),
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
            Self::Hardcoded(ERC20Index::new(byte))
        } else {
            let index = byte & !CUSTOM_FLAG; // remove the flag
            Self::Custom(ERC20Index::new(index))
        }
    }
}
