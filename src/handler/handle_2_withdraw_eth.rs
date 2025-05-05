use core::mem::MaybeUninit;

use crate::{
    eth, events, msg_sender,
    quantities::Atoms,
    state::TraderTokenKey,
    types::{Address, NATIVE_TOKEN},
};

pub const HANDLE_2_WITHDRAW_ETH: u8 = 2;
pub const HANDLE_2_PAYLOAD_LEN: usize = core::mem::size_of::<WithdrawETHParams>();

// We need packed otherwise the struct will be of size 32 not 28
// handle_1_credit_erc20 case is an exception. It worked without packed because 2 addresses
// equal to 40 bytes, a multiple of 8
#[repr(C, packed)]
struct WithdrawETHParams {
    /// Withdraw atoms to `recipient`. This allows a wallet to withdraw to another wallet
    pub recipient: Address,

    /// The atoms to withdraw. Raw atom to atom conversions should happen on client side.
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

pub fn handle_2_withdraw_eth(payload: &[u8]) -> Result<usize, ()> {
    if payload.len() < HANDLE_2_PAYLOAD_LEN {
        return Err(());
    }

    let params = unsafe { &*(payload.as_ptr() as *const WithdrawETHParams) };

    let mut sender_maybe = MaybeUninit::<Address>::uninit();
    let sender = unsafe {
        msg_sender(sender_maybe.as_mut_ptr() as *mut u8);
        sender_maybe.assume_init_ref()
    };

    let raw_atoms_withdrawn = events::withdraw(
        &TraderTokenKey {
            trader: *sender,
            token: NATIVE_TOKEN,
        },
        params.atoms,
    )?;

    eth::transfer_out(&params.recipient, &raw_atoms_withdrawn)?;

    Ok(HANDLE_2_PAYLOAD_LEN)
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

        let payload = WithdrawETHParams {
            recipient: msg_sender,
            atoms,
        };
        let payload_bytes: &[u8] = unsafe {
            core::slice::from_raw_parts(
                &payload as *const WithdrawETHParams as *const u8,
                core::mem::size_of::<WithdrawETHParams>(),
            )
        };

        let mut test_args: Vec<u8> = vec![];
        let num_calls: u8 = 1;
        test_args.push(num_calls);
        test_args.push(HANDLE_2_WITHDRAW_ETH);
        test_args.extend_from_slice(payload_bytes);
        set_test_args(test_args.clone());

        // Set slot
        let key = &TraderTokenKey {
            trader: payload.recipient,
            token: NATIVE_TOKEN,
        };
        let slot = TraderTokenState::new(Atoms(0), atoms, decimals);
        unsafe {
            slot.store(key);
        }

        // No need to set return data. Call result is success by default

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

        let atoms = Atoms(1);

        let payload = WithdrawETHParams {
            recipient: msg_sender,
            atoms,
        };
        let payload_bytes: &[u8] = unsafe {
            core::slice::from_raw_parts(
                &payload as *const WithdrawETHParams as *const u8,
                core::mem::size_of::<WithdrawETHParams>(),
            )
        };

        let mut test_args: Vec<u8> = vec![];
        let num_calls: u8 = 1;
        test_args.push(num_calls);
        test_args.push(HANDLE_2_WITHDRAW_ETH);
        test_args.extend_from_slice(payload_bytes);
        set_test_args(test_args.clone());

        // Set slot
        let key = &TraderTokenKey {
            trader: payload.recipient,
            token: NATIVE_TOKEN,
        };
        let slot = TraderTokenState::new(Atoms::ZERO, Atoms::ZERO, 6);
        unsafe {
            slot.store(key);
        }

        // No need to set return data. Call result is success by default

        let result = user_entrypoint(test_args.len());
        assert_eq!(result, 0);

        let mut trader_token_state_maybe = MaybeUninit::<TraderTokenState>::uninit();
        let trader_token_state =
            unsafe { TraderTokenState::load(key, &mut trader_token_state_maybe) };

        assert_eq!(trader_token_state.atoms_free.0, 0);
        assert_eq!(trader_token_state.atoms_locked.0, 0);
    }
}
