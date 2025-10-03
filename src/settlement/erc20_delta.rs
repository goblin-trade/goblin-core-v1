use crate::{
    erc20,
    goblin_error::GoblinError,
    quantities::{QuantityOps, UnsidedAtoms},
    require,
    settlement::{CommonDelta, ERC20Input, ERC20Transfer, TransferDirection},
    state::{ERC20Store, ERC20StoreKey, SlotState},
    tokens::{Token, DynamicTokenIndex},
    types::Address,
    CONTRACT_ADDRESS,
};

/// ERC20 atoms due to be deducted, locked or transferred out on settlement
#[derive(Default, Clone, Copy)]
pub struct ERC20Delta {
    /// Amount of atoms pending transfer, as read from input payload.
    ///
    /// - If direction Deposit, the exact amount must be deposited or the transaction will revert.
    /// - For direction withdraw, amount MIN(available, transfer_due) is withdrawn.
    /// This allows max available amount to be transferred out by passing u64::MAX
    pub transfer_due: UnsidedAtoms,

    /// The direction of pending transfer
    pub direction: TransferDirection,

    pub common_delta: CommonDelta,
}

impl ERC20Delta {
    pub fn new<S: ERC20Transfer>(erc20_input: &ERC20Input<S>) -> Self {
        Self {
            transfer_due: erc20_input.amount,
            direction: S::DIRECTION,
            common_delta: CommonDelta::default(),
        }
    }

    /// Update locked and free atoms of the store by applying the common delta
    fn apply_common_delta(&self, store_mut: &mut ERC20Store) -> Option<()> {
        store_mut.atoms_locked = store_mut
            .atoms_locked
            .checked_add(self.common_delta.maker_locked)?
            .checked_sub(self.common_delta.cancel_unlocked)?
            .checked_sub(self.common_delta.taker_self_trade_unlocked)?;

        store_mut.atoms_free = store_mut
            .atoms_free
            .checked_add(self.common_delta.free_atoms_out()?)?
            .checked_sub(self.common_delta.free_atoms_in()?)?;

        Some(())
    }

    pub fn settle(
        &self,
        index: DynamicTokenIndex,
        custom_token_list: &[Address],
        msg_sender: &Address,
        recipient: Option<&Address>,
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

                self.apply_common_delta(store_mut)
                    .ok_or(GoblinError::Overflow);

                match self.direction {
                    TransferDirection::Deposit => {
                        store_mut.atoms_free += self.transfer_due;
                        store_mut.store(&key);

                        let raw_atoms_in = self.transfer_due.to_raw_atoms(store_mut.decimals)?;
                        erc20::transfer_from(
                            token.address(),
                            msg_sender,
                            &CONTRACT_ADDRESS,
                            &raw_atoms_in,
                        )?;

                        Ok(())
                    }
                    TransferDirection::Withdraw => {
                        let withdraw_amount = store_mut.atoms_free.min(self.transfer_due);
                        store_mut.atoms_free -= withdraw_amount;
                        store_mut.store(&key);

                        if withdraw_amount == UnsidedAtoms::ZERO {
                            return Ok(());
                        }

                        if withdraw_internally {
                            match recipient {
                                Some(recipient) => {
                                    require!(
                                        *recipient != *msg_sender,
                                        GoblinError::NoInternalSelfWithdraw
                                    );

                                    let key = ERC20StoreKey::new(recipient, token.address());
                                    let mut recipient_store = ERC20Store::load(&key);
                                    let recipient_store_mut = recipient_store.as_mut();

                                    recipient_store_mut.atoms_free += withdraw_amount;
                                    recipient_store_mut.store(&key);
                                }
                                None => {
                                    return Err(GoblinError::NoInternalSelfWithdraw);
                                }
                            }
                        } else {
                            let raw_atoms_out = withdraw_amount.to_raw_atoms(store_mut.decimals)?;

                            let to = match recipient {
                                Some(recipient) => recipient,
                                None => msg_sender,
                            };

                            erc20::transfer(token.address(), to, &raw_atoms_out)?;
                        }

                        Ok(())
                    }
                }
            }
            Token::Eth => Err(GoblinError::ERC20NotETH),
        }
    }
}
