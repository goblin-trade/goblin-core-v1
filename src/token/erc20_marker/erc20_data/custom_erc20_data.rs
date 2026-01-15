use crate::{erc20, goblin_error::GoblinError, token::ERC20Data, types::Address};

/// Data for custom ERC20 tokens read from runtime
///
/// Address is passed through args while decimals are read via hostio
pub struct CustomERC20Data {
    pub address: Address,
}

impl ERC20Data for CustomERC20Data {
    fn address(&self) -> &Address {
        &self.address
    }

    fn decimals(&self) -> Result<u8, GoblinError> {
        erc20::decimals(&self.address)
    }
}

impl PartialEq for CustomERC20Data {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}
