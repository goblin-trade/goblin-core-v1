use crate::{erc20, goblin_error::GoblinError, types::Address};

#[derive(PartialEq)]
pub enum Token {
    /// The native gas token
    Eth,

    /// ERC20 token
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
