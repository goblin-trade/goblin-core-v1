use core::{
    mem::MaybeUninit,
    ops::{Add, Sub},
};

use crate::{
    eth,
    goblin_error::GoblinError,
    quantities::{Atoms, Delta},
    state::{SlotState, TraderTokenKey, TraderTokenState},
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
    pub eth_withdrawal_due: Delta,
}

impl EthDelta {
    pub fn execute_deposit(&mut self, amount: Atoms) -> Result<(), GoblinError> {
        self.slot_deduction_due = self.slot_deduction_due.sub(amount)?;
        Ok(())
    }

    pub fn execute_withdraw(&mut self, amount: Atoms) -> Result<(), GoblinError> {
        self.slot_deduction_due = self.slot_deduction_due.add(amount)?;
        self.eth_withdrawal_due = self.eth_withdrawal_due.add(amount)?;
        Ok(())
    }

    pub fn settle(&mut self, msg_sender: &Address) -> Result<(), GoblinError> {
        let trader_token_key = &TraderTokenKey::native_key(msg_sender);
        let mut trader_token_state_maybe = MaybeUninit::<TraderTokenState>::uninit();
        let trader_token_state =
            unsafe { TraderTokenState::load(trader_token_key, &mut trader_token_state_maybe) };

        // TODO don't store NATIVE_TOKEN_DECIMALS for ETH?
        trader_token_state.decimals = NATIVE_TOKEN_DECIMALS;

        // 1. Update TraderTokenState
        // If slot funds are insufficient then the TX will revert with AtomUnderflow error
        trader_token_state.atoms_free =
            trader_token_state.atoms_free.sub(self.slot_deduction_due)?;

        unsafe {
            trader_token_state.store(trader_token_key);
        }

        // 2. Transfer ETH out
        // native_withdrawal_due cannot be negative
        debug_assert!(self.eth_withdrawal_due >= Delta::ZERO);

        if self.eth_withdrawal_due > Delta::ZERO {
            let atoms_out = self.eth_withdrawal_due.abs();
            let raw_atoms_out = atoms_out.to_raw_atoms(NATIVE_TOKEN_DECIMALS)?;
            eth::transfer_out(msg_sender, &raw_atoms_out)?;
        }

        Ok(())
    }
}
