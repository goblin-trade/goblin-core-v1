use crate::{
    eth,
    goblin_error::GoblinError,
    hostio,
    quantities::Atoms,
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
    /// The amount is transferred out internally (store credit) or externally (transfer call).
    /// If recipient is the same as msg_sender, then this amount which was subtracted
    /// from atoms_free is credited back.
    pub withdrawal_due: Atoms,

    pub common_delta: CommonDelta,
    // /// Delta consumed by taker orders, due for subtraction from EthStore
    // consumed_by_engine: AtomsDelta,

    // /// Delta locked in maker orders. Positive if tokens are locked in maker orders,
    // /// negative if unlocked by cancelled orders
    // locked_by_engine: AtomsDelta,
}

// impl DeltaAccumulator for EthDelta {
//     fn add_consumed_amount(&mut self, consumed: AtomsDelta) -> Result<(), GoblinError> {
//         self.consumed_by_engine = self.consumed_by_engine.checked_add(consumed)?;
//         Ok(())
//     }

//     fn add_locked_amount(&mut self, locked: AtomsDelta) -> Result<(), GoblinError> {
//         self.locked_by_engine = self.locked_by_engine.checked_add(locked)?;
//         Ok(())
//     }
// }

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
    /// or to transfer it out
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

        self.settle_for_sender(store_mut)?;
        self.settle_for_recipient(msg_sender, store_mut, recipient, withdraw_internally)?;

        store_mut.store(&key);

        Ok(())
    }

    /// Apply delta on ETHStore
    fn settle_for_sender(&self, store_mut: &mut EthStore) -> Result<(), GoblinError> {
        let free_credit = store_mut
            .atoms_free
            .checked_add(self.common_delta.free_atoms_out()?)?;

        let free_debit = self
            .withdrawal_due
            .checked_add(self.common_delta.free_atoms_in()?)?;

        store_mut.atoms_free = free_credit.checked_sub(free_debit)?;

        store_mut.atoms_locked = store_mut
            .atoms_locked
            .checked_add(self.common_delta.maker_locked)?
            .checked_sub(self.common_delta.cancel_unlocked)?;

        Ok(())
    }

    /// Credit withdrawal_due to recipient
    ///
    /// Crediting is overflow unsafe
    ///
    /// # Arguments
    ///
    /// * `msg_sender`
    /// * `store_mut`
    /// * `recipient`- Recipient address
    /// * `withdraw_internally` - Whether to credit to TraderTokenState or to transfer out ETH
    fn settle_for_recipient(
        &self,
        msg_sender: &Address,
        msg_sender_store: &mut EthStore,
        recipient: Option<&Address>,
        withdraw_internally: bool,
    ) -> Result<(), GoblinError> {
        if self.withdrawal_due == Atoms::ZERO {
            return Ok(());
        }

        let credit = self.withdrawal_due;

        if withdraw_internally {
            match recipient {
                None => {
                    // Credit to msg_sender
                    msg_sender_store.atoms_free += credit;
                }
                Some(recipient_addr) if *recipient_addr == *msg_sender => {
                    // Credit to msg_sender (recipient is same as sender)
                    msg_sender_store.atoms_free += credit;
                }
                Some(recipient_addr) => {
                    // Credit to different recipient
                    let key = EthStoreKey::new(recipient_addr);
                    let mut recipient_store = EthStore::load(&key);
                    let recipient_store_mut = recipient_store.as_mut();

                    recipient_store_mut.atoms_free += credit;
                    recipient_store_mut.store(&key);
                }
            }
        } else {
            let raw_atoms_out = credit.to_raw_atoms(NATIVE_TOKEN_DECIMALS)?;

            let to = match recipient {
                Some(recipient) => recipient,
                None => msg_sender,
            };

            eth::transfer_out(to, &raw_atoms_out)?;
        }

        Ok(())
    }
}
