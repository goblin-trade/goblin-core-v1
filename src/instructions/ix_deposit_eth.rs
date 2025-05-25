use core::mem::MaybeUninit;

use crate::{
    goblin_error::GoblinError,
    hostio,
    quantities::{Atoms, RawAtoms},
    settlement::EthDelta,
    types::NATIVE_TOKEN_DECIMALS,
};

pub fn ix_deposit_eth(native_token_delta: &mut EthDelta) -> Result<(), GoblinError> {
    let mut msg_value_maybe = MaybeUninit::<RawAtoms>::uninit();
    let msg_value = unsafe {
        hostio::msg_value(msg_value_maybe.as_mut_ptr() as *mut u8);
        msg_value_maybe.assume_init_ref()
    };

    let atoms = Atoms::from_raw_atoms(msg_value, NATIVE_TOKEN_DECIMALS)?;
    native_token_delta.execute_deposit(atoms)?;

    Ok(())
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
