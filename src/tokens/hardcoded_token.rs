use crate::{goblin_error::GoblinError, tokens::ERC20Token, types::Address};

/// Token hardcoded into the smart contract
///
/// The token address and decimal count is hardcoded
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

impl ERC20Token for HardcodedToken {
    fn address(&self) -> &Address {
        &self.address
    }

    fn decimals(&self) -> Result<u8, GoblinError> {
        Ok(self.decimals)
    }
}
