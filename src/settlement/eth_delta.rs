use core::ops::{Add, Sub};

use crate::{
    eth,
    goblin_error::GoblinError,
    quantities::{Atoms, Delta},
    state::{TraderTokenKey, TraderTokenState},
    types::{Address, NATIVE_TOKEN_DECIMALS},
};

/// Eth atoms due to be deducted from slot and to be transferred out on settlement
///
/// EthDelta is tracked separately from TokenDeltaList because
/// * There is no address to track
/// * `native_withdrawal_due` can only have positive sign. ETH deposits
/// happen a-priori via msg.value, not during settlement.
///
/// Arithmetic on delta should be safe. Revert if any transaction overflows or underflows.
///
#[derive(Default)]
pub struct EthDelta {
    /// atoms due to be deducted from TraderTokenState (slot) on settlement
    ///
    /// * Positive: Deduct from TraderTokenState on settlement
    /// * Negative: add to TraderTokenState on settlement
    ///
    /// When tokens are used up to place orders, increase the delta. This delta
    /// must be squared off from TraderTokenState. Conversely if delta is negative,
    /// square off by crediting atoms to TraderTokenState
    ///
    /// TraderTokenState should have sufficient balance to cover slot_deduction_due
    /// on settlement, else the TX will revert due to insufficient funds.
    pub slot_deduction_due: Delta,

    /// atoms due to be transferred out to trader's ETH balance on settlement
    pub withdrawal_due: Delta,
}

impl EthDelta {
    pub fn execute_deposit(&mut self, amount: Atoms) -> Result<(), GoblinError> {
        self.slot_deduction_due = self.slot_deduction_due.sub(amount)?;
        // Do not subtract from withdrawal_due. ETH was already transferred via msg.value
        Ok(())
    }

    /// Execute the withdrawal
    pub fn execute_withdraw(&mut self, amount: Atoms) -> Result<(), GoblinError> {
        self.slot_deduction_due = self.slot_deduction_due.add(amount)?;
        self.withdrawal_due = self.withdrawal_due.add(amount)?;
        Ok(())
    }

    /// Settle, i.e. update the trader's token state and transfer ETH out
    ///
    /// * `slot_deduction_due` is applied on TraderTokenState(msg_sender)
    /// * Shortfall is deducted from `withdrawal_due`, i.e. less tokens are transferrred
    /// out if slot balance is insufficient.
    /// * `withdrawal_due` is transferred to `recipient`.
    /// * `transfer_to_recipient_internally` allows funds to be credited internally
    /// to TraderTokenState(recipient)
    ///
    pub fn settle(
        &mut self,
        msg_sender: &Address,
        recipient: &Address,
        transfer_to_recipient_internally: bool,
    ) -> Result<(), GoblinError> {
        let shortfall = TraderTokenState::update_free_atoms_and_store(
            &TraderTokenKey::native_key(msg_sender),
            NATIVE_TOKEN_DECIMALS,
            self.slot_deduction_due,
        )?;
        self.withdrawal_due -= shortfall;

        // 2. Transfer ETH out
        // There is no transfer in case for ETH, i.e. native_withdrawal_due cannot be negative
        debug_assert!(self.withdrawal_due >= Delta::ZERO);

        if self.withdrawal_due > Delta::ZERO {
            if transfer_to_recipient_internally {
                // No shortfall case because balance is added
                TraderTokenState::update_free_atoms_and_store(
                    &TraderTokenKey::native_key(recipient),
                    NATIVE_TOKEN_DECIMALS,
                    self.withdrawal_due.rev(),
                )?;
            } else {
                let atoms_out = self.withdrawal_due.abs();
                let raw_atoms_out = atoms_out.to_raw_atoms(NATIVE_TOKEN_DECIMALS)?;
                eth::transfer_out(recipient, &raw_atoms_out)?;
            }
        }

        Ok(())
    }
}
