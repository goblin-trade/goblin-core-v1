use crate::{goblin_error::GoblinError, token::ERC20Data, types::Address};

/// Data for hardcoded ERC20 tokens
pub struct HardcodedERC20Data {
    pub address: Address,
    pub decimals: u8,
}

impl ERC20Data for HardcodedERC20Data {
    fn address(&self) -> &Address {
        &self.address
    }

    fn decimals(&self) -> Result<u8, GoblinError> {
        Ok(self.decimals)
    }
}
