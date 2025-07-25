use crate::{erc20, goblin_error::GoblinError, tokens::HARDCODED_TOKENS, types::Address};

pub enum Token {
    Eth,
    ERC20(ERC20Token),
}

/// A generic token type to represent custom and hardcoded ERC20 tokens.
/// Decimal places are already set for hardcoded tokens, whereas we need to fetch them for custom tokens.
#[derive(Clone, Copy, PartialEq)]
pub enum ERC20Token {
    Custom(Address),
    Hardcoded(HardcodedToken),
}

/// A wrapper type for hardcoded tokens that contains both the address and decimals.
#[derive(Clone, Copy)]
pub struct HardcodedToken {
    pub address: Address,
    pub decimals: u8,
}

impl PartialEq for HardcodedToken {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}

impl ERC20Token {
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
            ERC20Token::Custom(token) => token,
            ERC20Token::Hardcoded(token) => &token.address,
        }
    }

    /// Returns the decimals of the token
    pub fn decimals(&self) -> Result<u8, GoblinError> {
        match self {
            ERC20Token::Custom(token) => erc20::decimals(token),
            ERC20Token::Hardcoded(token) => Ok(token.decimals),
        }
    }
}
