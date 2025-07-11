use crate::{erc20, goblin_error::GoblinError, tokens::HARDCODED_TOKENS, types::Address};

/// A generic token type to represent custom and hardcoded tokens.
/// Decimal places are already set for hardcoded tokens, whereas we need to fetch them for custom tokens.
#[derive(Clone, Copy)]
pub enum Token {
    Custom(Address),
    Hardcoded(HardcodedToken),
}

/// A wrapper type for hardcoded tokens that contains both the address and decimals.
#[derive(Clone, Copy)]
pub struct HardcodedToken {
    pub address: Address,
    pub decimals: u8,
}

impl Token {
    /// Get token by index
    ///
    /// * Custom tokens begin at index 0
    /// * Hardcoded tokens begin at index 127
    pub fn get_token_by_index(
        custom_token_list: &[Address],
        index: usize,
    ) -> Result<Self, GoblinError> {
        if index < custom_token_list.len() {
            let token = custom_token_list[index];
            Ok(Self::Custom(token))
        } else if index > 127 && index < (127 + HARDCODED_TOKENS.len()) {
            let token = HARDCODED_TOKENS[index - 127];
            Ok(Self::Hardcoded(token))
        } else {
            Err(GoblinError::NoTokenAtIndex)
        }
    }

    /// Returns the address of the token
    pub fn address(&self) -> &Address {
        match self {
            Token::Custom(token) => token,
            Token::Hardcoded(token) => &token.address,
        }
    }

    /// Returns the decimals of the token
    pub fn decimals(&self) -> Result<u8, GoblinError> {
        match self {
            Token::Custom(token) => erc20::decimals(token),
            Token::Hardcoded(token) => Ok(token.decimals),
        }
    }
}
