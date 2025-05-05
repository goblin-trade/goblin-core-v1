use core::mem::MaybeUninit;

use crate::{erc20, events, hostio, quantities::Atoms, state::TraderTokenKey, types::Address};

pub const HANDLE_3_WITHDRAW_ERC20: u8 = 3;
pub const HANDLE_3_PAYLOAD_LEN: usize = core::mem::size_of::<WithdrawERC20Params>();

#[repr(C, packed)]
struct WithdrawERC20Params {
    /// The token to withdraw
    pub token: Address,

    /// Withdraw raw atoms to `recipient`. This allows a wallet to withdraw to another wallet.
    pub recipient: Address,

    /// The raw to withdraw. Raw atom to atom conversions should happen on client side.
    ///
    /// If the value is greater than than the deposited amount, entire deposit
    /// is withdrawn.
    ///
    /// The atom bytes should be encoded in **little endian** for zero copy deserialization.
    ///
    /// For 1 atom
    /// - Correct (little endian, non ABI): 0x0100000000000000 = [0x01, 0x00, ...]
    /// - Wrong (big endian, ABI style): 0x0000000000000001 = [0x00, 0x00, ..., 0x01]
    pub atoms: Atoms,
}

pub fn handle_3_withdraw_erc20(payload: &[u8]) -> Result<usize, ()> {
    if payload.len() < HANDLE_3_PAYLOAD_LEN {
        return Err(());
    }

    let params = unsafe { &*(payload.as_ptr() as *const WithdrawERC20Params) };

    let mut sender_maybe = MaybeUninit::<Address>::uninit();
    let sender = unsafe {
        hostio::msg_sender(sender_maybe.as_mut_ptr() as *mut u8);
        sender_maybe.assume_init_ref()
    };

    let raw_atoms_withdrawn = events::withdraw(
        &TraderTokenKey {
            trader: *sender,
            token: params.token,
        },
        params.atoms,
    )?;

    erc20::transfer(&params.token, &params.recipient, &raw_atoms_withdrawn)?;

    Ok(HANDLE_3_PAYLOAD_LEN)
}

#[cfg(test)]
mod tests {
    use crate::{
        hostio::*,
        state::{SlotState, TraderTokenState},
        user_entrypoint,
    };
    use hex_literal::hex;

    use super::*;

    #[test]
    fn test_withdraw_sufficient_funds() {
        // Set hostios
        let msg_sender = hex!("3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E");
        set_msg_sender(msg_sender);

        let decimals = 6;
        let atoms = Atoms(1);

        let token = hex!("7E32b54800705876d3b5cFbc7d9c226a211F7C1a");

        let payload = WithdrawERC20Params {
            token,
            recipient: msg_sender,
            atoms,
        };
        let payload_bytes: &[u8] = unsafe {
            core::slice::from_raw_parts(
                &payload as *const WithdrawERC20Params as *const u8,
                core::mem::size_of::<WithdrawERC20Params>(),
            )
        };

        let mut test_args: Vec<u8> = vec![];
        let num_calls: u8 = 1;
        test_args.push(num_calls);
        test_args.push(HANDLE_3_WITHDRAW_ERC20);
        test_args.extend_from_slice(payload_bytes);
        set_test_args(test_args.clone());

        // Set slot
        let key = &TraderTokenKey {
            trader: payload.recipient,
            token,
        };
        let slot = TraderTokenState::new(Atoms::ZERO, atoms, decimals);
        unsafe {
            slot.store(key);
        }

        // Set return data to true
        let mut return_data = vec![0u8; 32];
        return_data[31] = 1;
        set_return_data(vec![return_data]);

        let result = user_entrypoint(test_args.len());
        assert_eq!(result, 0);

        let mut trader_token_state_maybe = MaybeUninit::<TraderTokenState>::uninit();
        let trader_token_state =
            unsafe { TraderTokenState::load(key, &mut trader_token_state_maybe) };

        assert_eq!(trader_token_state.atoms_free.0, 0);
        assert_eq!(trader_token_state.atoms_locked.0, 0);
    }

    #[test]
    fn test_withdraw_insufficient_funds() {
        // Set hostios
        let msg_sender = hex!("3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E");
        set_msg_sender(msg_sender);

        let decimals = 6;
        let atoms = Atoms(1);

        let token = hex!("7E32b54800705876d3b5cFbc7d9c226a211F7C1a");

        let payload = WithdrawERC20Params {
            token,
            recipient: msg_sender,
            atoms,
        };
        let payload_bytes: &[u8] = unsafe {
            core::slice::from_raw_parts(
                &payload as *const WithdrawERC20Params as *const u8,
                core::mem::size_of::<WithdrawERC20Params>(),
            )
        };

        let mut test_args: Vec<u8> = vec![];
        let num_calls: u8 = 1;
        test_args.push(num_calls);
        test_args.push(HANDLE_3_WITHDRAW_ERC20);
        test_args.extend_from_slice(payload_bytes);
        set_test_args(test_args.clone());

        // Set slot
        let key = &TraderTokenKey {
            trader: payload.recipient,
            token,
        };
        let slot = TraderTokenState::new(Atoms::ZERO, Atoms::ZERO, decimals);
        unsafe {
            slot.store(key);
        }

        // Set return data to false
        let return_data = vec![0u8; 32];
        set_return_data(vec![return_data]);

        let result = user_entrypoint(test_args.len());
        assert_eq!(result, 1);

        let mut trader_token_state_maybe = MaybeUninit::<TraderTokenState>::uninit();
        let trader_token_state =
            unsafe { TraderTokenState::load(key, &mut trader_token_state_maybe) };

        assert_eq!(trader_token_state.atoms_free.0, 0);
        assert_eq!(trader_token_state.atoms_locked.0, 0);
    }
}
