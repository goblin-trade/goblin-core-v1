use core::marker::PhantomData;

use crate::{erc20, goblin_error::GoblinError, tokens::HARDCODED_TOKENS, types::Address};

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

#[derive(Clone, Copy, PartialEq)]
pub struct CustomToken {
    pub address: Address,
}

pub trait ERC20TokenTrait {
    fn address(&self) -> &Address;
    fn decimals(&self) -> Result<u8, GoblinError>;

    // Incase we need a common function, otherwise remove
    // fn decode_index(index: TokenIndex<Self>, custom_erc20_list: &[Address])
    // where
    //     Self: Sized;
}

impl ERC20TokenTrait for HardcodedToken {
    fn address(&self) -> &Address {
        &self.address
    }

    fn decimals(&self) -> Result<u8, GoblinError> {
        Ok(self.decimals)
    }
}

impl ERC20TokenTrait for CustomToken {
    fn address(&self) -> &Address {
        &self.address
    }

    fn decimals(&self) -> Result<u8, GoblinError> {
        erc20::decimals(&self.address)
    }
}

#[derive(Clone, Copy)]
pub struct TokenIndex<T: ERC20TokenTrait> {
    pub inner: u8,
    _marker: PhantomData<T>,
}

impl<T: ERC20TokenTrait> TokenIndex<T> {
    pub const fn new(inner: u8) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }
}

impl TokenIndex<HardcodedToken> {
    fn get_token(&self) -> Option<&HardcodedToken> {
        HARDCODED_TOKENS.get(self.inner as usize)
    }
}

impl TokenIndex<CustomToken> {
    fn get_token<'a>(&self, custom_erc20_list: &'a [CustomToken]) -> Option<&'a CustomToken> {
        custom_erc20_list.get(self.inner as usize)
    }
}

/// Unifying type to lookup ERC20 token address by index
#[derive(Clone, Copy)]
pub enum DynamicIndex {
    Hardcoded(TokenIndex<HardcodedToken>),
    Custom(TokenIndex<CustomToken>),
}
