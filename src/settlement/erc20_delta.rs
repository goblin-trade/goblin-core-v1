use crate::{
    goblin_error::GoblinError,
    quantities::{Atoms, Delta},
    types::Address,
};

#[derive(Clone, Copy)]
pub struct ERC20Delta {
    /// The token index
    pub index: u8,

    /// The token address as read from hardcoded and custom lists
    pub address: Address,

    /// Amount of atoms pending withdrawal, as read from input payload
    pub withdrawal_due: Delta,

    /// Delta consumed by taker orders, due for subtraction from ERC20Store
    consumed_by_engine: Delta,

    /// Atoms locked in limit orders
    locked_by_engine: Atoms,
}

impl ERC20Delta {
    pub fn new(index: u8, address: Address, withdrawal_due: Delta) -> Self {
        Self {
            index,
            address,
            withdrawal_due,
            consumed_by_engine: Delta::ZERO,
            locked_by_engine: Atoms::ZERO,
        }
    }

    pub fn add_consumed_amount(&mut self, consumed: Delta) -> Result<(), GoblinError> {
        self.consumed_by_engine = self.consumed_by_engine.checked_add(consumed)?;
        Ok(())
    }

    pub fn settle(&self) -> Result<(), GoblinError> {
        Ok(())
    }
}
