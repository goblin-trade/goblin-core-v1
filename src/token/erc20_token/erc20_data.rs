use crate::{goblin_error::GoblinError, types::Address};

pub trait ERC20Data {
    /// Token address
    fn address(&self) -> &Address;

    /// Token decimals
    fn decimals(&self) -> Result<u8, GoblinError>;
}
