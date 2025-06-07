use core::ops::Add;

use crate::{
    erc20,
    goblin_error::GoblinError,
    quantities::{Atoms, Delta},
    require,
    state::{ERC20Store, ERC20StoreKey, SlotStateV2},
    types::Address,
    CONTRACT_ADDRESS,
};

/// ERC20 atoms due to be deducted, locked or transferred out on settlement
#[derive(Clone, Copy)]
pub struct ERC20Delta {
    /// The token index
    pub index: u8,

    /// The token address as read from hardcoded and custom lists
    pub address: Address,

    /// Amount of atoms pending withdrawal, as read from input payload.
    ///
    /// Unlike EthDelta, withdrawal_due is of type Delta intead of Atoms.
    /// It can be negative to indicate a pending deposit.
    pub withdrawal_due: Delta,

    /// Delta consumed by taker orders, due for subtraction from ERC20Store
    consumed_by_engine: Delta,

    /// Delta locked in maker orders. Positive if tokens are locked in maker orders,
    /// negative if unlocked by cancelled orders
    locked_by_engine: Delta,
}

impl ERC20Delta {
    pub fn new(index: u8, address: Address, withdrawal_due: Delta) -> Self {
        Self {
            index,
            address,
            withdrawal_due,
            consumed_by_engine: Delta::ZERO,
            locked_by_engine: Delta::ZERO,
        }
    }

    pub fn add_consumed_amount(&mut self, consumed: Delta) -> Result<(), GoblinError> {
        self.consumed_by_engine = self.consumed_by_engine.checked_add(consumed)?;
        Ok(())
    }

    pub fn add_locked_amount(&mut self, locked: Delta) -> Result<(), GoblinError> {
        self.locked_by_engine = self.locked_by_engine.checked_add(locked)?;
        Ok(())
    }

    fn debit_due(&self) -> Result<Delta, GoblinError> {
        self.consumed_by_engine
            .checked_add(self.locked_by_engine)?
            .checked_add(self.withdrawal_due)
    }

    pub fn settle(&self) -> Result<(), GoblinError> {
        Ok(())
    }

    /// Settle the balance for msg.sender.
    ///
    /// In case of shortfall, msg.sender must pay
    ///
    /// * Unlike EthDelta, this step can transfer in ERC20 tokens if withdrawal_due
    /// is negative
    ///
    /// * If deposit_shortfall is true and ERC20Store cannot cover the debit due,
    /// the shortfall is subtracted from withdrawal_due
    pub fn settle_for_sender(
        &mut self,
        msg_sender: &Address,
        deposit_shortfall: bool,
    ) -> Result<(), GoblinError> {
        let key = ERC20StoreKey::new(msg_sender, &self.address);
        let mut store = ERC20Store::load(&key);

        // Update locked atoms
        let initial_locked = store.as_ref().atoms_locked;
        store.as_mut().atoms_locked = initial_locked.add(self.locked_by_engine)?;

        // Update free atoms
        let initial_free = store.as_ref().atoms_free;
        let initial_free_delta = initial_free.to_delta()?;
        let free_after_debit_delta = initial_free_delta.checked_sub(self.debit_due()?)?;

        if free_after_debit_delta >= Delta::ZERO {
            store.as_mut().atoms_free = free_after_debit_delta.abs();
        } else {
            require!(deposit_shortfall, GoblinError::ShortfallDepositNotAllowed);

            // Subtract shortfall from withdrawal_due
            // If withdrawal_due becomes negative, msg.sender will transfer in tokens to
            // cover the shortfall
            store.as_mut().atoms_free = Atoms::ZERO;
            self.withdrawal_due = self.withdrawal_due.checked_sub(free_after_debit_delta)?;
        }

        // Store to slot
        store.as_ref().store(&key);

        Ok(())
    }

    fn settle_for_recipient(
        &self,
        msg_sender: &Address,
        recipient: &Address,
        withdraw_internally: bool,
    ) -> Result<(), GoblinError> {
        if self.withdrawal_due == Delta::ZERO {
            return Ok(());
        }

        // Decimals are needed in all 3 cases
        // - To transfer in negative withdrawal_due
        // - To trasfer out withdrawal_due externally
        // - If transfer out to internal recipient. Since new recipient addresses
        // can be used we need to store decimal places in there
        //
        // We should read decimals in settle_for_sender() because decimal places
        // must be stored in ERC20Store of msg.sender in case we're accessing it for
        // the first time
        let decimals = 18;
        let amount = self.withdrawal_due.abs().to_raw_atoms(decimals)?;

        if self.withdrawal_due > Delta::ZERO {
            if withdraw_internally {
                let key = ERC20StoreKey::new(msg_sender, &self.address);
                let mut store = ERC20Store::load(&key);

                let initial_balance = store.as_ref().atoms_free;
                store.as_mut().atoms_free =
                    initial_balance.checked_add(self.withdrawal_due.abs())?;
                store.as_mut().decimals = decimals;

                store.as_ref().store(&key);

                Ok(())
            } else {
                erc20::transfer(&self.address, recipient, &amount)
            }
        } else {
            erc20::transfer_from(&self.address, msg_sender, &CONTRACT_ADDRESS, &amount)
        }
    }
}
