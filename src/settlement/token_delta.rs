use core::{
    mem::MaybeUninit,
    ops::{Add, Sub},
};

use crate::{
    goblin_error::GoblinError,
    quantities::{Atoms, Delta},
    state::{SlotState, TraderTokenKey, TraderTokenState},
    types::Address,
};

/// ERC20 atoms due to be deducted from slot and to be transferred out on settlement
#[derive(Default)]
pub struct TokenDelta {
    /// The token address
    pub address: Address,

    /// atoms due to be deducted from TraderTokenState (slot) on settlement
    ///
    /// * Positive: Deduct from TraderTokenState
    /// * Negative: add to TraderTokenState
    ///
    /// When tokens are used up to place orders, increase the delta. This delta
    /// must be squared off from TraderTokenState. Conversely if delta is negative,
    /// square off by crediting atoms to TraderTokenState
    ///
    /// TraderTokenState should have sufficient balance to cover slot_deduction_due
    /// on settlement, else the TX will revert due to insufficient funds.
    pub slot_deduction_due: Delta,

    /// atoms due to be transferred out to trader's ERC20 account on settlement
    pub withdrawal_due: Delta,
}

impl TokenDelta {
    pub fn execute_deposit(&mut self, amount: Atoms) -> Result<(), GoblinError> {
        self.slot_deduction_due = self.slot_deduction_due.sub(amount)?;
        self.withdrawal_due = self.withdrawal_due.sub(amount)?;
        Ok(())
    }

    pub fn execute_withdraw(&mut self, amount: Atoms) -> Result<(), GoblinError> {
        self.slot_deduction_due = self.slot_deduction_due.add(amount)?;
        self.withdrawal_due = self.withdrawal_due.add(amount)?;
        Ok(())
    }
}
