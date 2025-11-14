use core::mem::MaybeUninit;

use crate::{
    quantities::{DeltaAtoms, QuantityOps},
    settlement::CommonDelta,
    state::ERC20Store,
    tokens::{CustomToken, ERC20TokenTrait, HardcodedToken, TokenIndex, HARDCODED_TOKENS},
};

pub const MAX_CUSTOM_DELTAS: usize = 8;

pub type HardcodedTokenDeltas = [ERC20DeltaMaybe; HARDCODED_TOKENS.len()];
pub type CustomTokenDeltas = [ERC20DeltaMaybe; MAX_CUSTOM_DELTAS];

impl Default for HardcodedTokenDeltas {
    fn default() -> Self {
        [ERC20DeltaMaybe::default(); HARDCODED_TOKENS.len()]
    }
}

pub trait DeltaListTrait<T: ERC20TokenTrait> {
    fn get_delta(&self, token_index: TokenIndex<T>) -> &ERC20DeltaMaybe;
}

impl DeltaListTrait<HardcodedToken> for HardcodedTokenDeltas {
    fn get_delta(&self, token_index: TokenIndex<HardcodedToken>) -> &ERC20DeltaMaybe {
        &self[token_index.inner as usize]
    }
}

impl DeltaListTrait<CustomToken> for CustomTokenDeltas {
    fn get_delta(&self, token_index: TokenIndex<CustomToken>) -> &ERC20DeltaMaybe {
        &self[token_index.inner as usize]
    }
}

#[derive(Clone, Copy)]
pub struct ERC20DeltaMaybe {
    /// Whether the delta is initialized
    pub init: bool,
    pub inner: MaybeUninit<ERC20Delta>,
}

impl Default for ERC20DeltaMaybe {
    fn default() -> Self {
        Self {
            init: false,
            inner: MaybeUninit::uninit(),
        }
    }
}

/// ERC20 atoms due to be deducted, locked or transferred out on settlement
#[derive(Default, Clone, Copy)]
pub struct ERC20Delta {
    pub deposit_due: DeltaAtoms,

    pub common_delta: CommonDelta,
}

impl ERC20Delta {
    // TODO function to add to deposit_due

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

    // pub fn settle(
    //     &self,
    //     index: DynamicIndex,
    //     custom_token_list: &[Address],
    //     msg_sender: &Address,
    //     recipient: Option<&Address>,
    //     withdraw_internally: bool,
    // ) -> Result<(), GoblinError> {
    //     match index.to_token(custom_token_list)? {
    //         Token::ERC20(token) => {
    //             let key = ERC20StoreKey::new(msg_sender, token.address());
    //             let mut store = ERC20Store::load(&key);
    //             let store_mut = store.as_mut();

    //             // If ERC20 store was read for the first time, fetch and store token decimals
    //             if store_mut.is_empty() {
    //                 store_mut.decimals = token.decimals()?;
    //             }

    //             self.apply_common_delta(store_mut)
    //                 .ok_or(GoblinError::Overflow);

    //             match self.direction {
    //                 TransferDirection::Deposit => {
    //                     store_mut.atoms_free += self.transfer_due;
    //                     store_mut.store(&key);

    //                     let raw_atoms_in = self.transfer_due.to_raw_atoms(store_mut.decimals)?;
    //                     erc20::transfer_from(
    //                         token.address(),
    //                         msg_sender,
    //                         &CONTRACT_ADDRESS,
    //                         &raw_atoms_in,
    //                     )?;

    //                     Ok(())
    //                 }
    //                 TransferDirection::Withdraw => {
    //                     let withdraw_amount = store_mut.atoms_free.min(self.transfer_due);
    //                     store_mut.atoms_free -= withdraw_amount;
    //                     store_mut.store(&key);

    //                     if withdraw_amount == UnsidedAtoms::ZERO {
    //                         return Ok(());
    //                     }

    //                     if withdraw_internally {
    //                         match recipient {
    //                             Some(recipient) => {
    //                                 require!(
    //                                     *recipient != *msg_sender,
    //                                     GoblinError::NoInternalSelfWithdraw
    //                                 );

    //                                 let key = ERC20StoreKey::new(recipient, token.address());
    //                                 let mut recipient_store = ERC20Store::load(&key);
    //                                 let recipient_store_mut = recipient_store.as_mut();

    //                                 recipient_store_mut.atoms_free += withdraw_amount;
    //                                 recipient_store_mut.store(&key);
    //                             }
    //                             None => {
    //                                 return Err(GoblinError::NoInternalSelfWithdraw);
    //                             }
    //                         }
    //                     } else {
    //                         let raw_atoms_out = withdraw_amount.to_raw_atoms(store_mut.decimals)?;

    //                         let to = match recipient {
    //                             Some(recipient) => recipient,
    //                             None => msg_sender,
    //                         };

    //                         erc20::transfer(token.address(), to, &raw_atoms_out)?;
    //                     }

    //                     Ok(())
    //                 }
    //             }
    //         }
    //         Token::Eth => Err(GoblinError::ERC20NotETH),
    //     }
    // }
}
