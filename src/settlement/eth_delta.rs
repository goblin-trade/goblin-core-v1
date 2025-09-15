use crate::{
    eth,
    goblin_error::GoblinError,
    hostio,
    quantities::Atoms,
    require,
    settlement::CommonDelta,
    state::{EthStore, EthStoreKey, SlotState},
    types::{Address, NATIVE_TOKEN_DECIMALS},
};

/// ETH atoms due to be deducted, locked or transferred out on settlement
pub struct EthDelta {
    /// Atoms credited by msg.value
    pub msg_value_atoms: Atoms,

    /// Amount of atoms pending withdrawal, as read from input payload.
    ///
    /// The actual amount withdrawn is MIN(available, widthdrawal_due)
    /// This allows us to withdraw max available amount by passing u64::MAX
    ///
    /// The amount is transferred out internally (store credit) or externally (transfer call).
    pub withdrawal_due: Atoms,

    pub common_delta: CommonDelta,
}

impl EthDelta {
    pub fn init(
        track_msg_value: bool,
        eth_withdrawal_due: Option<&Atoms>,
    ) -> Result<Self, GoblinError> {
        let msg_value_atoms = if track_msg_value {
            let msg_value = hostio::msg_value();
            Atoms::from_raw_atoms(msg_value.as_ref(), NATIVE_TOKEN_DECIMALS)?
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
            common_delta: CommonDelta::default(),
        })
    }

    /// Settle, i.e. update the trader's token state and transfer ETH out
    ///
    /// # Arguments
    ///
    /// * `msg_sender` - Earns msg_value_atoms and pays for the delta due
    /// * `recipient` - Receives `withdrawal_due`
    /// * `withdraw_internally` - Whether to credit ETH to the recipient's TraderTokenState
    /// or to transfer it out. If this is true, recipient cannot be None or equal to msg_sender adress.
    /// it must be a different address.
    ///
    pub fn settle(
        &self,
        msg_sender: &Address,
        recipient: Option<&Address>,
        withdraw_internally: bool,
    ) -> Result<(), GoblinError> {
        let key = EthStoreKey::new(msg_sender);
        let mut store = EthStore::load(&key);
        let store_mut = store.as_mut();

        // Update locked
        store_mut.atoms_locked = store_mut
            .atoms_locked
            .checked_add(self.common_delta.maker_locked)?
            .checked_sub(self.common_delta.cancel_unlocked)?
            .checked_sub(self.common_delta.taker_self_trade_unlocked)?;

        // Update free
        store_mut.atoms_free = store_mut
            .atoms_free
            .checked_add(self.common_delta.free_atoms_out()?)?
            .checked_sub(self.common_delta.free_atoms_in()?)?;

        // Deduct withdraw amount
        let withdraw_amount = store_mut.atoms_free.min(self.withdrawal_due);
        store_mut.atoms_free -= withdraw_amount;
        store_mut.store(&key);

        if withdraw_amount == Atoms::ZERO {
            return Ok(());
        }

        if withdraw_internally {
            match recipient {
                Some(recipient) => {
                    require!(
                        *recipient != *msg_sender,
                        GoblinError::NoInternalSelfWithdraw
                    );

                    let key = EthStoreKey::new(recipient);
                    let mut recipient_store = EthStore::load(&key);
                    let recipient_store_mut = recipient_store.as_mut();

                    recipient_store_mut.atoms_free += withdraw_amount;
                    recipient_store_mut.store(&key);
                }
                None => {
                    return Err(GoblinError::NoInternalSelfWithdraw);
                }
            }
        } else {
            let raw_atoms_out = withdraw_amount.to_raw_atoms(NATIVE_TOKEN_DECIMALS)?;

            let to = match recipient {
                Some(recipient) => recipient,
                None => msg_sender,
            };

            eth::transfer_out(to, &raw_atoms_out)?;
        }

        Ok(())
    }
}
