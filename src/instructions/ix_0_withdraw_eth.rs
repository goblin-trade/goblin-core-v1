use crate::{goblin_error::GoblinError, quantities::Atoms, require, settlement::EthDelta};

pub const IX_0_WITHDRAW_ETH: u8 = 0;
pub const IX_0_PAYLOAD_LEN: usize = core::mem::size_of::<Atoms>();

/// Credit atoms to withdraw to delta
pub fn ix_0_withdraw_eth(payload: &[u8], eth_delta: &mut EthDelta) -> Result<usize, GoblinError> {
    require!(
        payload.len() >= IX_0_PAYLOAD_LEN,
        GoblinError::InvalidPayload
    );

    let atoms = unsafe { &*(payload.as_ptr() as *const Atoms) };
    eth_delta.execute_withdraw(*atoms)?;

    Ok(IX_0_PAYLOAD_LEN)
}

// #[cfg(test)]
// mod tests {
//     use crate::{
//         hostio::*,
//         state::{SlotState, TraderTokenState},
//         user_entrypoint,
//     };
//     use hex_literal::hex;

//     use super::*;

//     #[test]
//     fn test_withdraw_sufficient_funds() {
//         // Set hostios
//         let msg_sender = hex!("3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E");
//         set_msg_sender(msg_sender);

//         let decimals = 6;
//         let atoms = Atoms(1);

//         let payload = WithdrawETHParams {
//             recipient: msg_sender,
//             atoms,
//         };
//         let payload_bytes: &[u8] = unsafe {
//             core::slice::from_raw_parts(
//                 &payload as *const WithdrawETHParams as *const u8,
//                 core::mem::size_of::<WithdrawETHParams>(),
//             )
//         };

//         let mut test_args: Vec<u8> = vec![];
//         let num_calls: u8 = 1;
//         test_args.push(num_calls);
//         test_args.push(IX_0_WITHDRAW_ETH);
//         test_args.extend_from_slice(payload_bytes);
//         set_test_args(test_args.clone());

//         // Set slot
//         let key = &TraderTokenKey {
//             trader: payload.recipient,
//             token: NATIVE_TOKEN,
//         };
//         let slot = TraderTokenState::new(Atoms(0), atoms, decimals);
//         unsafe {
//             slot.store(key);
//         }

//         // No need to set return data. Call result is success by default

//         let result = user_entrypoint(test_args.len());
//         assert_eq!(result, 0);

//         let mut trader_token_state_maybe = MaybeUninit::<TraderTokenState>::uninit();
//         let trader_token_state =
//             unsafe { TraderTokenState::load(key, &mut trader_token_state_maybe) };

//         assert_eq!(trader_token_state.atoms_free.0, 0);
//         assert_eq!(trader_token_state.atoms_locked.0, 0);
//     }

//     #[test]
//     fn test_withdraw_insufficient_funds() {
//         // Set hostios
//         let msg_sender = hex!("3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E");
//         set_msg_sender(msg_sender);

//         let atoms = Atoms(1);

//         let payload = WithdrawETHParams {
//             recipient: msg_sender,
//             atoms,
//         };
//         let payload_bytes: &[u8] = unsafe {
//             core::slice::from_raw_parts(
//                 &payload as *const WithdrawETHParams as *const u8,
//                 core::mem::size_of::<WithdrawETHParams>(),
//             )
//         };

//         let mut test_args: Vec<u8> = vec![];
//         let num_calls: u8 = 1;
//         test_args.push(num_calls);
//         test_args.push(IX_0_WITHDRAW_ETH);
//         test_args.extend_from_slice(payload_bytes);
//         set_test_args(test_args.clone());

//         // Set slot
//         let key = &TraderTokenKey {
//             trader: payload.recipient,
//             token: NATIVE_TOKEN,
//         };
//         let slot = TraderTokenState::new(Atoms::ZERO, Atoms::ZERO, 6);
//         unsafe {
//             slot.store(key);
//         }

//         // No need to set return data. Call result is success by default

//         let result = user_entrypoint(test_args.len());
//         assert_eq!(result, 0);

//         let mut trader_token_state_maybe = MaybeUninit::<TraderTokenState>::uninit();
//         let trader_token_state =
//             unsafe { TraderTokenState::load(key, &mut trader_token_state_maybe) };

//         assert_eq!(trader_token_state.atoms_free.0, 0);
//         assert_eq!(trader_token_state.atoms_locked.0, 0);
//     }
// }
