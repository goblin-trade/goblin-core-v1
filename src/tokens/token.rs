use crate::{
    erc20,
    goblin_error::GoblinError,
    quantities::Atoms,
    state::{ERC20Store, ERC20StoreKey, EthStore, EthStoreKey, SlotState},
    types::Address,
};

#[derive(PartialEq)]
pub enum Token {
    /// The native gas token
    Eth,

    /// ERC20 token
    ERC20(ERC20Token),
}

impl Token {
    /// Unlocked matched tokens for a maker
    /// Since resting orders are backed by locked tokens, we can subtract directly.
    pub fn unlock_matched_atoms(&mut self, trader: &Address, unlocked: Atoms) {
        match self {
            Token::Eth => {
                let key = EthStoreKey::new(trader);
                let mut store = EthStore::load(&key);

                store.as_mut().atoms_locked -= unlocked.into();
                store.as_mut().store(&key);
            }
            Token::ERC20(erc20_token) => {
                let key = ERC20StoreKey::new(trader, erc20_token.address());
                let mut store = ERC20Store::load(&key);

                store.as_mut().atoms_locked -= unlocked.into();
                store.as_mut().store(&key);
            }
        }
    }
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
