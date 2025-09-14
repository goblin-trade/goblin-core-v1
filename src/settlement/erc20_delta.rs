use core::{marker::PhantomData, ops::Add};

use crate::{
    erc20,
    goblin_error::GoblinError,
    quantities::Atoms,
    require,
    settlement::{CommonDelta, DeltaAccumulator, ERC20Transfer, TransferDirection},
    state::{ERC20Store, ERC20StoreKey, SlotState},
    tokens::{ERC20Token, Token, TokenIndex},
    types::Address,
    CONTRACT_ADDRESS,
};

/// Generic struct for ERC20 deposits or withdrawals
#[repr(C, packed)]
pub struct ERC20Input<S> {
    pub index: TokenIndex,
    pub amount: Atoms,
    _marker: core::marker::PhantomData<S>,
}

/// ERC20 atoms due to be deducted, locked or transferred out on settlement
#[derive(Default, Clone, Copy)]
pub struct ERC20Delta {
    /// Amount of atoms pending transfer, as read from input payload.
    pub transfer_due: Atoms,

    /// The direction of pending transfer
    pub direction: TransferDirection,

    pub common_delta: CommonDelta,
    // /// Delta consumed by taker orders, due for subtraction from ERC20Store
    // consumed_by_engine: AtomsDelta,

    // /// Delta locked in maker orders. Positive if tokens are locked in maker orders,
    // /// negative if unlocked by cancelled orders
    // locked_by_engine: AtomsDelta,
}

impl ERC20Delta {
    pub fn new<S: ERC20Transfer>(erc20_input: &ERC20Input<S>) -> Self {
        Self {
            transfer_due: erc20_input.amount,
            direction: S::DIRECTION,
            common_delta: CommonDelta::default(),
        }
    }

    fn debit_due(&self) -> Result<AtomsDelta, GoblinError> {
        self.consumed_by_engine
            .checked_add(self.locked_by_engine)?
            .checked_add(self.withdrawal_due)
    }

    pub fn settle(
        &mut self,
        index: TokenIndex,
        custom_token_list: &[Address],
        msg_sender: &Address,
        recipient: Option<&Address>,
        deposit_shortfall: bool,
        withdraw_internally: bool,
    ) -> Result<(), GoblinError> {
        match index.to_token(custom_token_list)? {
            Token::ERC20(token) => {
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
            Token::Eth => Err(GoblinError::ERC20NotETH),
        }
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
        // Update locked atoms
        let initial_locked = msg_sender_store.atoms_locked;
        msg_sender_store.atoms_locked = initial_locked.add(self.locked_by_engine)?;

        let initial_free = msg_sender_store.atoms_free.to_delta()?;
        let free_after_debit = initial_free.checked_sub(self.debit_due()?)?;

        if free_after_debit >= AtomsDelta::ZERO {
            msg_sender_store.atoms_free = free_after_debit.abs();
        } else {
            require!(deposit_shortfall, GoblinError::ShortfallDepositNotAllowed);

            // Subtract shortfall from withdrawal_due
            msg_sender_store.atoms_free = Atoms::ZERO;
            self.withdrawal_due = self.withdrawal_due.checked_sub(free_after_debit)?;
        }

        // Transfer in tokens if withdrawal_due is negative
        if self.withdrawal_due < AtomsDelta::ZERO {
            let debit = self.withdrawal_due.abs();
            let debit_raw_atoms = debit.to_raw_atoms(msg_sender_store.decimals)?;
            erc20::transfer_from(
                token.address(),
                msg_sender,
                &CONTRACT_ADDRESS,
                &debit_raw_atoms,
            )?;
            self.withdrawal_due = AtomsDelta::ZERO;
        }

        Ok(())
    }

    /// Settle delta for recipient
    ///
    /// * Transfer out tokens to recipient if withdrawal_due is greater than 0.
    /// * Negative withdrawal_due is illegal. It is already handled in settle_for_sender()
    ///
    /// TODO refactor- calling with negative withdrawal_due is illegal
    fn settle_for_recipient(
        &self,
        token: &ERC20Token,
        msg_sender: &Address,
        msg_sender_store: &mut ERC20Store,
        recipient: Option<&Address>,
        withdraw_internally: bool,
    ) -> Result<(), GoblinError> {
        debug_assert!(self.withdrawal_due >= AtomsDelta::ZERO);

        if self.withdrawal_due == AtomsDelta::ZERO {
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
