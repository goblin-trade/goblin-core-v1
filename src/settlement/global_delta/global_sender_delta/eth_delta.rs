use crate::{
    eth,
    goblin_error::GoblinError,
    quantities::{QuantityOps, UnsidedAtoms},
    require,
    settlement::global_delta::CommonDelta,
    state::{EthStore, EthStoreKey, SlotState},
    types::{Address, NATIVE_TOKEN_DECIMALS},
};

/// ETH atoms due to be deducted, locked or transferred out on settlement
pub struct EthDelta {
    /// Atoms credited by msg.value
    pub msg_value: UnsidedAtoms,

    /// Amount of ETH atoms pending withdrawal, as read from global namespace header
    ///
    /// The actual amount withdrawn is MIN(available, widthdrawal_due)
    /// This allows us to withdraw max available amount by passing u64::MAX
    ///
    /// The amount is transferred out internally (store credit) or externally (transfer call).
    pub eth_out_due: UnsidedAtoms,

    pub common_delta: CommonDelta,
}

impl EthDelta {
    pub fn new(msg_value: UnsidedAtoms, eth_out_due: UnsidedAtoms) -> Self {
        Self {
            msg_value,
            eth_out_due,
            common_delta: CommonDelta::default(),
        }
    }

    // /// Update locked and free atoms of the store by applying the common delta
    // fn apply_common_delta(&self, store_mut: &mut EthStore) -> Option<()> {
    //     store_mut.atoms_locked = store_mut
    //         .atoms_locked
    //         .checked_add(self.common_delta.maker_locked)?
    //         .checked_sub(self.common_delta.cancel_unlocked)?
    //         .checked_sub(self.common_delta.taker_self_trade_unlocked)?;

    //     store_mut.atoms_free = store_mut
    //         .atoms_free
    //         .checked_add(self.common_delta.free_atoms_out()?)?
    //         .checked_sub(self.common_delta.free_atoms_in()?)?;

    //     Some(())
    // }

    // /// Settle, i.e. update the trader's token state and transfer ETH out
    // ///
    // /// # Arguments
    // ///
    // /// * `msg_sender` - Earns msg_value_atoms and pays for the delta due
    // /// * `recipient` - Receives `withdrawal_due`
    // /// * `withdraw_internally` - Whether to credit ETH to the recipient's TraderTokenState
    // /// or to transfer it out. If this is true, recipient cannot be None or equal to msg_sender adress.
    // /// it must be a different address.
    // ///
    // pub fn settle(
    //     &self,
    //     msg_sender: &Address,
    //     recipient: Option<&Address>,
    //     withdraw_internally: bool,
    // ) -> Result<(), GoblinError> {
    //     let key = EthStoreKey::new(msg_sender);
    //     let mut store = EthStore::load(&key);
    //     let store_mut = store.as_mut();

    //     self.apply_common_delta(store_mut)
    //         .ok_or(GoblinError::Overflow)?;

    //     // Deduct withdraw amount
    //     let withdraw_amount = store_mut.atoms_free.min(self.eth_out_due);
    //     store_mut.atoms_free -= withdraw_amount;
    //     store_mut.store(&key);

    //     if withdraw_amount == UnsidedAtoms::ZERO {
    //         return Ok(());
    //     }

    //     if withdraw_internally {
    //         match recipient {
    //             Some(recipient) => {
    //                 require!(
    //                     *recipient != *msg_sender,
    //                     GoblinError::NoInternalSelfWithdraw
    //                 );

    //                 let key = EthStoreKey::new(recipient);
    //                 let mut recipient_store = EthStore::load(&key);
    //                 let recipient_store_mut = recipient_store.as_mut();

    //                 recipient_store_mut.atoms_free += withdraw_amount;
    //                 recipient_store_mut.store(&key);
    //             }
    //             None => {
    //                 return Err(GoblinError::NoInternalSelfWithdraw);
    //             }
    //         }
    //     } else {
    //         let raw_atoms_out = withdraw_amount.to_raw_atoms(NATIVE_TOKEN_DECIMALS)?;

    //         let to = match recipient {
    //             Some(recipient) => recipient,
    //             None => msg_sender,
    //         };

    //         eth::transfer_out(to, &raw_atoms_out)?;
    //     }

    //     Ok(())
    // }
}
