use crate::{
    goblin_error::GoblinError,
    require,
    token::{CustomIndex, CustomToken, HardcodedIndex, TokenIndex, HARDCODED_TOKENS},
    types::Address,
};

/// Enum type of hardcoded and custom token indices.
///
/// Allows custom markets to use both hardcoded and custom tokens
#[derive(Clone, Copy, PartialEq)]
pub enum DynamicIndex {
    Hardcoded(HardcodedIndex),
    Custom(CustomIndex),
}

impl DynamicIndex {
    /// Decode a token index byte into either a hardcoded or custom token index.
    /// Hardcoded token indices are validated. Therefore we can do `TokenIndex<HardcodedToken>::get_token()`
    /// without safety checks.
    ///
    /// Convention:
    /// - If the MSB (bit 7) is 0 → Hardcoded token index (0–127)
    /// - If the MSB (bit 7) is 1 → Custom token index (0–127, but stored as 128–255)
    pub fn new(byte: u8) -> Result<Self, GoblinError> {
        const CUSTOM_FLAG: u8 = 0b1000_0000;
        if (byte & CUSTOM_FLAG) == 0 {
            // Hardcoded token
            let index = byte;
            require!(
                (index as usize) < HARDCODED_TOKENS.len(),
                GoblinError::InvalidHardcodedTokenIndex
            );

            Ok(Self::Hardcoded(TokenIndex::new(index)))
        } else {
            // Custom token
            let index = byte & !CUSTOM_FLAG; // remove the flag
            Ok(Self::Custom(TokenIndex::new(index)))
        }
    }

    /// Get the token address corresponding to the index. If it is a custom token, this
    /// address is read from the custom token list
    pub fn address(&self, custom_erc20_list: &[CustomToken]) -> Result<Address, GoblinError> {
        let address = match self {
            DynamicIndex::Hardcoded(hardcoded_token_index) => {
                let token = hardcoded_token_index.get_token();
                token.address
            }
            DynamicIndex::Custom(custom_token_index) => {
                let token = custom_token_index
                    .get_token(custom_erc20_list)
                    .ok_or(GoblinError::InvalidCustomTokenIndex)?;
                token.address
            }
        };

        Ok(address)
    }
}
