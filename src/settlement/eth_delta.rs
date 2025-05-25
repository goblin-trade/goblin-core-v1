use core::{
    mem::MaybeUninit,
    ops::{Add, Sub},
};

use crate::{
    eth,
    goblin_error::GoblinError,
    hostio,
    quantities::{Atoms, Delta, RawAtoms},
    require,
    state::{TraderTokenKey, TraderTokenState},
    types::{Address, NATIVE_TOKEN_DECIMALS},
};

/// Eth atoms due to be deducted from slot and to be transferred out on settlement
///
/// EthDelta is tracked separately from TokenDeltaList because
/// * There is no address to track
/// * `native_withdrawal_due` can only have positive sign. ETH deposits
/// happen a-priori via msg.value, not during settlement.
///
/// Arithmetic on delta should be safe. Revert if any transaction overflows or underflows.
///
#[derive(Default)]
pub struct EthDelta {
    /// atoms due to be deducted from TraderTokenState (slot) on settlement
    ///
    /// * Positive: Deduct from TraderTokenState on settlement
    /// * Negative: add to TraderTokenState on settlement
    ///
    /// When tokens are used up to place orders, increase the delta. This delta
    /// must be squared off from TraderTokenState. Conversely if delta is negative,
    /// square off by crediting atoms to TraderTokenState
    ///
    /// TraderTokenState should have sufficient balance to cover slot_deduction_due
    /// on settlement, else the TX will revert due to insufficient funds.
    pub slot_deduction_due: Delta,

    /// atoms due to be transferred out to trader's ETH balance on settlement
    pub withdrawal_due: Delta,
}

impl EthDelta {
    /// Update ETH delta if `track_eth_delta` is true
    ///
    /// - Credit msg.value
    /// - Read `delta` from input bytes and subtract. Since credit happens only with
    /// msg.value, negative values of `delta` are a no-op
    pub fn update_eth_delta(
        &mut self,
        track_eth_delta: bool,
        start_index: usize,
        input: &[u8; 512],
        len: usize,
    ) -> Result<(), GoblinError> {
        if !track_eth_delta {
            return Ok(());
        }

        let total_len = start_index + 8;
        require!(len >= total_len, GoblinError::InvalidPayload);

        // Add positive delta
        // Negative or zero delta is no-op
        let delta = unsafe { &*(input[start_index..(start_index + 8)].as_ptr() as *const Delta) };
        if *delta > Delta::ZERO {
            self.slot_deduction_due = self.slot_deduction_due.checked_add(*delta)?;
            self.withdrawal_due = self.withdrawal_due.checked_add(*delta)?;
        }

        // Read msg.value and subtract from delta
        let mut msg_value_maybe = MaybeUninit::<RawAtoms>::uninit();
        let msg_value = unsafe {
            hostio::msg_value(msg_value_maybe.as_mut_ptr() as *mut u8);
            msg_value_maybe.assume_init_ref()
        };

        let atoms_in = Atoms::from_raw_atoms(msg_value, NATIVE_TOKEN_DECIMALS)?;
        self.slot_deduction_due = self.slot_deduction_due.sub(atoms_in)?;

        Ok(())
    }

    /// Settle, i.e. update the trader's token state and transfer ETH out
    ///
    /// * `slot_deduction_due` is applied on TraderTokenState(msg_sender)
    /// * Shortfall is deducted from `withdrawal_due`, i.e. less tokens are transferrred
    /// out if slot balance is insufficient.
    /// * `withdrawal_due` is transferred to `recipient`.
    /// * `withdraw_internally` allows funds to be credited internally
    /// to TraderTokenState(recipient)
    ///
    pub fn settle(
        &mut self,
        msg_sender: &Address,
        recipient: &Address,
        withdraw_internally: bool,
    ) -> Result<(), GoblinError> {
        let shortfall = TraderTokenState::update_free_atoms_and_store(
            &TraderTokenKey::native_key(msg_sender),
            NATIVE_TOKEN_DECIMALS,
            self.slot_deduction_due,
        )?;

        // ETH shortfall cannot be a-posteriori deposited, therefore deduct
        self.withdrawal_due -= shortfall;

        // 2. Transfer ETH out
        // There is no transfer in case for ETH, i.e. native_withdrawal_due cannot be negative
        debug_assert!(self.withdrawal_due >= Delta::ZERO);

        if self.withdrawal_due > Delta::ZERO {
            if withdraw_internally {
                // No shortfall case because balance is added
                TraderTokenState::update_free_atoms_and_store(
                    &TraderTokenKey::native_key(recipient),
                    NATIVE_TOKEN_DECIMALS,
                    self.withdrawal_due.rev(),
                )?;
            } else {
                let atoms_out = self.withdrawal_due.abs();
                let raw_atoms_out = atoms_out.to_raw_atoms(NATIVE_TOKEN_DECIMALS)?;
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
