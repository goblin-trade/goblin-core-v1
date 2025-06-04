use core::{mem::MaybeUninit, ops::Add};

use crate::{
    eth,
    goblin_error::GoblinError,
    hostio::hostio_msg_value,
    quantities::{Atoms, Delta},
    require,
    state::{EthStore, EthStoreKey, SlotKeyV2, SlotState, TraderTokenKey, TraderTokenState},
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

    fn settle_for_sender_v2(&self, msg_sender: &Address) -> Result<(), GoblinError> {
        let key = &EthStoreKey {
            trader: *msg_sender,
        };

        // Looks cleaner but keccak is run twice
        let eth_store = key.read_slot();
        key.write_slot(eth_store.as_ref());

        Ok(())
    }

    /// Add or deduct delta from TraderTokenState for msg.sender
    fn settle_for_sender(&self, msg_sender: &Address) -> Result<(), GoblinError> {
        let key = &TraderTokenKey::native_key(msg_sender);
        let mut trader_token_state_maybe = MaybeUninit::<TraderTokenState>::uninit();
        let trader_token_state =
            unsafe { TraderTokenState::load(key, &mut trader_token_state_maybe) };

        let atoms_free = (trader_token_state.atoms_free + self.msg_value_atoms).to_delta()?;
        let atoms_due_delta = self.consumed_by_engine.add(self.withdrawal_due)?;

        require!(
            atoms_free >= atoms_due_delta,
            GoblinError::CannotDepositEthOnSettlement
        );
        trader_token_state.atoms_free = atoms_free.checked_sub(atoms_due_delta)?.abs();

        trader_token_state.decimals = NATIVE_TOKEN_DECIMALS;
        unsafe {
            trader_token_state.store(key);
        }
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
        if self.withdrawal_due > Atoms::ZERO {
            if withdraw_internally {
                TraderTokenState::add_free_atoms_and_store(
                    &TraderTokenKey::native_key(recipient),
                    self.withdrawal_due,
                    NATIVE_TOKEN_DECIMALS,
                );
            } else {
                let raw_atoms_out = self.withdrawal_due.to_raw_atoms(NATIVE_TOKEN_DECIMALS)?;
                eth::transfer_out(recipient, &raw_atoms_out)?;
            }
        }

        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use core::i64;

//     use crate::{
//         hostio::set_test_args,
//         state::{SlotState, TraderTokenKey, TraderTokenState},
//         user_entrypoint_inner,
//     };

//     use super::*;
//     use hex_literal::hex;

//     #[test]
//     fn test_deposit_ignored_if_no_flag() {
//         // Set hostios
//         let msg_sender = hex!("3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E");
//         hostio::set_msg_sender(msg_sender);

//         // 10^12 raw atoms is rounded to 1 atom
//         let eth_raw_atoms: u128 = 1_000_000_000_000;
//         let mut value: [u8; 32] = [0u8; 32];
//         value[16..].copy_from_slice(&eth_raw_atoms.to_be_bytes());

//         hostio::set_msg_value(value);

//         // No deposit flag
//         let test_args: Vec<u8> = vec![0b0000_0000];
//         set_test_args(test_args.clone());

//         assert_eq!(user_entrypoint_inner(test_args.len()).unwrap(), ());

//         let trader_token_key = TraderTokenKey::native_key(&msg_sender);
//         let mut trader_token_state_maybe = MaybeUninit::<TraderTokenState>::uninit();
//         let trader_token_state =
//             unsafe { TraderTokenState::load(&trader_token_key, &mut trader_token_state_maybe) };

//         assert_eq!(trader_token_state.atoms_free.0, 0);
//     }

//     #[test]
//     fn test_deposit_1_atom_eth() {
//         // Set hostios
//         let msg_sender = hex!("3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E");
//         hostio::set_msg_sender(msg_sender);

//         // 10^12 raw atoms is rounded to 1 atom

//         let eth_raw_atoms: u128 = 1_000_000_000_000;
//         let mut value: [u8; 32] = [0u8; 32];
//         value[16..].copy_from_slice(&eth_raw_atoms.to_be_bytes());

//         hostio::set_msg_value(value);

//         let test_args: Vec<u8> = vec![0b0000_0001];
//         set_test_args(test_args.clone());

//         assert_eq!(user_entrypoint_inner(test_args.len()).unwrap(), ());

//         let trader_token_key = TraderTokenKey::native_key(&msg_sender);
//         let mut trader_token_state_maybe = MaybeUninit::<TraderTokenState>::uninit();
//         let trader_token_state =
//             unsafe { TraderTokenState::load(&trader_token_key, &mut trader_token_state_maybe) };

//         assert_eq!(trader_token_state.atoms_free.0, 1);
//     }

//     #[test]
//     fn test_dust_is_lost() {
//         // Set hostios
//         let msg_sender = hex!("3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E");
//         hostio::set_msg_sender(msg_sender);

//         // Less than 10^12
//         let eth_raw_atoms: u128 = 1_000_000_000_000 - 1;
//         let mut value: [u8; 32] = [0u8; 32];
//         value[16..].copy_from_slice(&eth_raw_atoms.to_be_bytes());

//         hostio::set_msg_value(value);

//         let test_args: Vec<u8> = vec![0b0000_0001];
//         set_test_args(test_args.clone());

//         assert_eq!(user_entrypoint_inner(test_args.len()).unwrap(), ());

//         let trader_token_key = TraderTokenKey::native_key(&msg_sender);
//         let mut trader_token_state_maybe = MaybeUninit::<TraderTokenState>::uninit();
//         let trader_token_state =
//             unsafe { TraderTokenState::load(&trader_token_key, &mut trader_token_state_maybe) };

//         assert_eq!(trader_token_state.atoms_free.0, 0);
//     }

//     #[test]
//     fn test_rounded_down_to_atom_size() {
//         // Set hostios
//         let msg_sender = hex!("3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E");
//         hostio::set_msg_sender(msg_sender);

//         // More than 10^12
//         let eth_raw_atoms: u128 = 1_000_000_000_000 + 1;
//         let mut value: [u8; 32] = [0u8; 32];
//         value[16..].copy_from_slice(&eth_raw_atoms.to_be_bytes());

//         hostio::set_msg_value(value);

//         let test_args: Vec<u8> = vec![0b0000_0001];
//         set_test_args(test_args.clone());

//         assert_eq!(user_entrypoint_inner(test_args.len()).unwrap(), ());

//         let trader_token_key = TraderTokenKey::native_key(&msg_sender);
//         let mut trader_token_state_maybe = MaybeUninit::<TraderTokenState>::uninit();
//         let trader_token_state =
//             unsafe { TraderTokenState::load(&trader_token_key, &mut trader_token_state_maybe) };

//         assert_eq!(trader_token_state.atoms_free.0, 1);
//     }

//     #[test]
//     fn test_at_max_legal_value() {
//         // Set hostios
//         let msg_sender = hex!("3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E");
//         hostio::set_msg_sender(msg_sender);

//         // 1. Max value of delta is i64::MAX
//         // Therefore max value of raw atoms is i64::MAX * 10^12
//         let max_legal_value = i64::MAX as u128 * 10u128.pow(12);

//         let mut value: [u8; 32] = [0u8; 32];
//         value[16..].copy_from_slice(&max_legal_value.to_be_bytes());

//         hostio::set_msg_value(value);

//         let test_args: Vec<u8> = vec![0b0000_0001];
//         set_test_args(test_args.clone());

//         assert_eq!(user_entrypoint_inner(test_args.len()).unwrap(), ());

//         let trader_token_key = TraderTokenKey::native_key(&msg_sender);
//         let mut trader_token_state_maybe = MaybeUninit::<TraderTokenState>::uninit();
//         let trader_token_state =
//             unsafe { TraderTokenState::load(&trader_token_key, &mut trader_token_state_maybe) };
//         assert_eq!(trader_token_state.atoms_free.0, i64::MAX as u64);
//     }

//     #[test]
//     fn test_delta_underflow_above_max_value() {
//         // Set hostios
//         let msg_sender = hex!("3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E");
//         hostio::set_msg_sender(msg_sender);

//         // 1. Max value of delta is i64::MAX
//         // Therefore max value of raw atoms is i64::MAX * 10^12
//         let max_legal_value = i64::MAX as u128 * 10u128.pow(12);

//         // Add 1 atom, i.e. 10^12 raw atoms
//         // Amounts smaller than 10^12 are ignored as dust (previous test)
//         let overflowing_value = max_legal_value + 10u128.pow(12);

//         let mut value: [u8; 32] = [0u8; 32];
//         value[16..].copy_from_slice(&overflowing_value.to_be_bytes());
//         hostio::set_msg_value(value);

//         let test_args: Vec<u8> = vec![0b0000_0001];
//         set_test_args(test_args.clone());

//         assert!(matches!(
//             user_entrypoint_inner(test_args.len()),
//             Err(GoblinError::DeltaUnderflow)
//         ));
//     }

//     #[test]
//     fn test_delta_underflow_with_max_eth_value() {
//         // Set hostios
//         let msg_sender = hex!("3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E");
//         hostio::set_msg_sender(msg_sender);

//         let value: [u8; 32] = [255u8; 32];
//         hostio::set_msg_value(value);

//         let test_args: Vec<u8> = vec![0b0000_0001];
//         set_test_args(test_args.clone());

//         assert!(matches!(
//             user_entrypoint_inner(test_args.len()),
//             Err(GoblinError::DeltaUnderflow)
//         ));
//     }
// }
