use core::ops::Add;

use crate::{
    erc20,
    goblin_error::GoblinError,
    quantities::{Atoms, Delta},
    require,
    settlement::DeltaAccumulator,
    state::{ERC20Store, ERC20StoreKey, SlotState},
    tokens::ERC20Token,
    types::Address,
    CONTRACT_ADDRESS,
};

/// Withdrawal due for an ERC20 token as read from the args.
/// It contains the token index instead of the token address.
#[repr(C, packed)]
pub struct IndexedERC20Delta {
    pub index: u8,
    pub withdrawal_due: Delta,
}

/// ERC20 atoms due to be deducted, locked or transferred out on settlement
#[derive(Clone, Copy)]
pub struct ERC20Delta {
    pub index: u8,
    // /// The token as read from hardcoded or custom list
    // pub token: Token,
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

impl DeltaAccumulator for ERC20Delta {
    fn add_consumed_amount(&mut self, consumed: Delta) -> Result<(), GoblinError> {
        self.consumed_by_engine = self.consumed_by_engine.checked_add(consumed)?;
        Ok(())
    }

    fn add_locked_amount(&mut self, locked: Delta) -> Result<(), GoblinError> {
        self.locked_by_engine = self.locked_by_engine.checked_add(locked)?;
        Ok(())
    }
}

impl ERC20Delta {
    pub fn new(index: u8, withdrawal_due: Delta) -> Self {
        Self {
            index,
            withdrawal_due,
            consumed_by_engine: Delta::ZERO,
            locked_by_engine: Delta::ZERO,
        }
    }

    fn debit_due(&self) -> Result<Delta, GoblinError> {
        self.consumed_by_engine
            .checked_add(self.locked_by_engine)?
            .checked_add(self.withdrawal_due)
    }

    pub fn settle(
        &mut self,
        custom_token_list: &[Address],
        msg_sender: &Address,
        recipient: Option<&Address>,
        deposit_shortfall: bool,
        withdraw_internally: bool,
    ) -> Result<(), GoblinError> {
        let token = ERC20Token::get_token_by_index(custom_token_list, self.index as usize)?;

        let key = ERC20StoreKey::new(msg_sender, token.address());
        let mut store = ERC20Store::load(&key);
        let store_mut = store.as_mut();

        // If ERC20 store was read for the first time, fetch and store token decimals
        if store_mut.is_empty() {
            store_mut.decimals = token.decimals()?;
        }

        self.settle_for_sender(&token, msg_sender, store_mut, deposit_shortfall)?;
        self.settle_for_recipient(
            &token,
            msg_sender,
            store_mut,
            recipient,
            withdraw_internally,
        )?;

        store_mut.store(&key);

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
    /// the shortfall is added to withdrawal_due
    pub fn settle_for_sender(
        &mut self,
        token: &ERC20Token,
        msg_sender: &Address,
        msg_sender_store: &mut ERC20Store,
        deposit_shortfall: bool,
    ) -> Result<(), GoblinError> {
        if msg_sender_store.is_empty() {
            msg_sender_store.decimals = token.decimals()?;
        }

        // Update locked atoms
        let initial_locked = msg_sender_store.atoms_locked;
        msg_sender_store.atoms_locked = initial_locked.add(self.locked_by_engine)?;

        let initial_free = msg_sender_store.atoms_free.to_delta()?;
        let free_after_debit = initial_free.checked_sub(self.debit_due()?)?;

        if free_after_debit >= Delta::ZERO {
            msg_sender_store.atoms_free = free_after_debit.abs();
        } else {
            require!(deposit_shortfall, GoblinError::ShortfallDepositNotAllowed);

            // Subtract shortfall from withdrawal_due
            msg_sender_store.atoms_free = Atoms::ZERO;
            self.withdrawal_due = self.withdrawal_due.checked_sub(free_after_debit)?;
        }

        // Transfer in tokens if withdrawal_due is negative
        if self.withdrawal_due < Delta::ZERO {
            let debit = self.withdrawal_due.abs();
            let debit_raw_atoms = debit.to_raw_atoms(msg_sender_store.decimals)?;
            erc20::transfer_from(
                token.address(),
                msg_sender,
                &CONTRACT_ADDRESS,
                &debit_raw_atoms,
            )?;
            self.withdrawal_due = Delta::ZERO;
        }

        Ok(())
    }

    /// Settle delta for recipient
    ///
    /// * Transfer out tokens to recipient if withdrawal_due is greater than 0.
    /// * Negative withdrawal_due is illegal. It is already handled in settle_for_sender()
    fn settle_for_recipient(
        &self,
        token: &ERC20Token,
        msg_sender: &Address,
        msg_sender_store: &mut ERC20Store,
        recipient: Option<&Address>,
        withdraw_internally: bool,
    ) -> Result<(), GoblinError> {
        debug_assert!(self.withdrawal_due >= Delta::ZERO);

        if self.withdrawal_due == Delta::ZERO {
            return Ok(());
        }

        let credit = self.withdrawal_due.abs();

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
                    let recipient_key = ERC20StoreKey::new(recipient_addr, token.address());
                    let mut recipient_store = ERC20Store::load(&recipient_key);
                    let recipient_store_mut = recipient_store.as_mut();

                    recipient_store_mut.atoms_free += credit;
                    recipient_store_mut.decimals = msg_sender_store.decimals;
                    recipient_store_mut.store(&recipient_key);
                }
            }

            Ok(())
        } else {
            let to = match recipient {
                Some(recipient) => recipient,
                None => msg_sender,
            };

            let credit_raw_atoms = credit.to_raw_atoms(msg_sender_store.decimals)?;
            erc20::transfer(token.address(), to, &credit_raw_atoms)
        }
    }
}
