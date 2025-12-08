use crate::{erc20, goblin_error::GoblinError, token::ERC20Marker, types::Address};

/// A custom token
///
/// Unlike an hardcoded token, the decimal places are read at runtime
/// with a hostio call to the token contract
#[derive(Clone, Copy, PartialEq)]
pub struct CustomToken {
    pub address: Address,
}

impl ERC20Marker for CustomToken {
    fn address(&self) -> &Address {
        &self.address
    }

    fn decimals(&self) -> Result<u8, GoblinError> {
        erc20::decimals(&self.address)
    }
}
