use core::ops::Add;

use crate::{
    eth,
    goblin_error::GoblinError,
    hostio::hostio_msg_value,
    quantities::{Atoms, Delta},
    require,
    state::{EthStore, EthStoreKey, SlotStateV2},
    types::{Address, NATIVE_TOKEN_DECIMALS},
};

/// Eth atoms due to be deducted from slot and to be transferred out on settlement
#[derive(Default)]
pub struct EthDelta {
    /// Atoms credited by msg.value
    pub msg_value_atoms: Atoms,

    /// atoms due to be withdrawn. Read from payload.
    pub withdrawal_due: Atoms,

    /// Delta consumed by matching engine. If value is negative then tokens were emitted
    /// instead of consumed.
    pub consumed_by_engine: Delta,
}

impl EthDelta {
    pub fn init(
        track_msg_value: bool,
        eth_withdrawal_due: Option<&Atoms>,
    ) -> Result<Self, GoblinError> {
        let msg_value_atoms = if track_msg_value {
            unsafe {
                let msg_value = hostio_msg_value();
                Atoms::from_raw_atoms(msg_value.as_ref(), NATIVE_TOKEN_DECIMALS)?
            }
        } else {
            Atoms::ZERO
        };

        let withdrawal_due = match eth_withdrawal_due {
            Some(eth_withdrawal_due) => *eth_withdrawal_due,
            None => Atoms::ZERO,
        };

        Ok(Self {
            msg_value_atoms,
            withdrawal_due,
            consumed_by_engine: Delta::ZERO,
        })
    }

    /// Settle, i.e. update the trader's token state and transfer ETH out
    ///
    /// # Arguments
    ///
    /// * `msg_sender` - Earns msg_value_atoms and pays for the delta due
    /// * `recipient` - Receives `withdrawal_due`
    /// * `withdraw_internally` - Whether to credit ETH to the recipient's TraderTokenState
    /// or to transfer it out
    ///
    pub fn settle(
        &self,
        msg_sender: &Address,
        recipient: &Address,
        withdraw_internally: bool,
    ) -> Result<(), GoblinError> {
        self.settle_for_sender(msg_sender)?;
        self.settle_for_recipient(recipient, withdraw_internally)?;

        Ok(())
    }

    /// Settle EthStore for msg.sender and write to state
    fn settle_for_sender(&self, msg_sender: &Address) -> Result<(), GoblinError> {
        let key = EthStoreKey::new(msg_sender);
        let mut store = EthStore::load(&key);

        let atoms_balance = (store.as_ref().atoms_free + self.msg_value_atoms).to_delta()?;
        let atoms_debit = self.consumed_by_engine.add(self.withdrawal_due)?;

        require!(
            atoms_balance >= atoms_debit,
            GoblinError::CannotDepositEthOnSettlement
        );
        store.as_mut().atoms_free = atoms_balance.checked_sub(atoms_debit)?.abs();

        store.as_ref().store(&key);

        Ok(())
    }

    /// Transfer withdrawal_due ETH atoms to the recipient
    ///
    /// # Arguments
    ///
    /// * `recipient`- Recipient address
    /// * `withdraw_internally` - Whether to credit to TraderTokenState or to transfer out ETH
    fn settle_for_recipient(
        &self,
        recipient: &Address,
        withdraw_internally: bool,
    ) -> Result<(), GoblinError> {
        if self.withdrawal_due == Atoms::ZERO {
            return Ok(());
        } else if withdraw_internally {
            let key = EthStoreKey::new(recipient);
            let mut eth_store = EthStore::load(&key);

            eth_store.as_mut().atoms_free += self.withdrawal_due;
            eth_store.as_ref().store(&key);
        } else {
            let raw_atoms_out = self.withdrawal_due.to_raw_atoms(NATIVE_TOKEN_DECIMALS)?;
            eth::transfer_out(recipient, &raw_atoms_out)?;
        }

        Ok(())
    }
}
