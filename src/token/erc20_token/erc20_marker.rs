use crate::{goblin_error::GoblinError, types::Address};

/// Trait for ERC20 tokens
pub trait ERC20Marker
where
    Self: Sized,
{
    /// Token address
    fn address(&self) -> &Address;

    /// Token decimals
    fn decimals(&self) -> Result<u8, GoblinError>;
}
