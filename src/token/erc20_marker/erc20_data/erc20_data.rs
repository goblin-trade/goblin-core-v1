use crate::{goblin_error::GoblinError, types::Address};

/// Getter trait to read ERC20 address and decimals from data stores
pub trait ERC20Data {
    /// Token address
    fn address(&self) -> &Address;

    /// Token decimals
    fn decimals(&self) -> Result<u8, GoblinError>;
}
