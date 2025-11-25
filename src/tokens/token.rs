use core::marker::PhantomData;

use crate::{
    erc20, goblin_error::GoblinError, require, settlement::global_delta::LazyERC20Delta,
    tokens::HARDCODED_TOKENS, types::Address,
};

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

pub trait ERC20TokenTrait
where
    Self: Sized,
{
    type DeltaList: AsRef<[LazyERC20Delta]> + AsMut<[LazyERC20Delta]>;
    fn delta(delta_list: &Self::DeltaList, index: TokenIndex<Self>) -> &LazyERC20Delta;

    fn address(&self) -> &Address;
    fn decimals(&self) -> Result<u8, GoblinError>;
}

impl ERC20TokenTrait for HardcodedToken {
    type DeltaList = [LazyERC20Delta; HARDCODED_TOKENS.len()];

    fn delta(delta_list: &Self::DeltaList, index: TokenIndex<Self>) -> &LazyERC20Delta {
        &delta_list[index.inner as usize]
    }

    fn address(&self) -> &Address {
        &self.address
    }

    fn decimals(&self) -> Result<u8, GoblinError> {
        Ok(self.decimals)
    }
}

impl ERC20TokenTrait for CustomToken {
    type DeltaList = [LazyERC20Delta; 8];

    fn delta(delta_list: &Self::DeltaList, index: TokenIndex<Self>) -> &LazyERC20Delta {
        &delta_list[index.inner as usize]
    }

    fn address(&self) -> &Address {
        &self.address
    }

    fn decimals(&self) -> Result<u8, GoblinError> {
        erc20::decimals(&self.address)
    }
}

#[derive(Clone, Copy, PartialEq)]
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
    // Return the hardcoded token address corresponding to this index
    //
    // Externally ensure that the token index is valid. This function does
    // not check for bounds.
    pub fn get_token(&self) -> &HardcodedToken {
        unsafe { HARDCODED_TOKENS.get_unchecked(self.inner as usize) }
    }
}

impl TokenIndex<CustomToken> {
    pub fn get_token<'a>(&self, custom_erc20_list: &'a [CustomToken]) -> Option<&'a CustomToken> {
        custom_erc20_list.get(self.inner as usize)
    }
}

// Short alias
pub type HardcodedIndex = TokenIndex<HardcodedToken>;

/// Unifying type to lookup ERC20 token address by index
#[derive(Clone, Copy, PartialEq)]
pub enum DynamicIndex {
    Hardcoded(TokenIndex<HardcodedToken>),
    Custom(TokenIndex<CustomToken>),
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

    pub fn address_bytes(
        &self,
        custom_erc20_list: &[CustomToken],
    ) -> Result<[u8; 20], GoblinError> {
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
