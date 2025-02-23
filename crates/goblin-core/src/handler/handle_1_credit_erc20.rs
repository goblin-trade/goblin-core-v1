use core::mem::MaybeUninit;

use crate::{
    erc20::transfer_from,
    log_i64, log_txt, msg_sender,
    quantities::{Atoms, Lots},
    state::{SlotState, TraderTokenKey, TraderTokenState},
    storage_flush_cache,
    types::Address,
    ADDRESS,
};

pub const HANDLE_1_CREDIT_ERC20: u8 = 1;

#[repr(C)]
struct CreditERC20Params {
    /// The token to credit
    pub token: Address,

    /// Credit input lots to `recipient`. This allows a wallet to fund another wallet
    pub recipient: Address,

    /// The lots to credit. Atom to lot conversions should happen on client side.
    ///
    /// The lots bytes should be encoded in **little endian** for zero copy deserialization.
    ///
    /// For 1 lot
    /// - Correct (little endian, non ABI): 0x0100000000000000 = [0x01, 0x00, ...]
    /// - Wrong (big endian, ABI style): 0x0000000000000001 = [0x00, 0x00, ..., 0x01]
    pub lots: Lots,
}

/// Credit an ERC20 token to a recipient
pub fn handle_1_credit_erc20(payload: &[u8]) -> i32 {
    if payload.len() != core::mem::size_of::<CreditERC20Params>() {
        return 1;
    }

    let params = unsafe { &*(payload.as_ptr() as *const CreditERC20Params) };

    let mut sender_maybe = MaybeUninit::<Address>::uninit();
    let sender = unsafe {
        msg_sender(sender_maybe.as_mut_ptr() as *mut u8);
        sender_maybe.assume_init_ref()
    };

    let atoms = Atoms::from(&params.lots);

    // Transfer tokens to smart contract, not params.recipient
    let result = transfer_from(&params.token, sender, &ADDRESS, &atoms);

    unsafe {
        let msg = b"Call result";
        log_txt(msg.as_ptr(), msg.len());
        log_i64(result as i64);
    }

    if result != 0 {
        return 1;
    }

    // Credit lots
    let key = &TraderTokenKey {
        trader: params.recipient,
        token: params.token,
    };

    let mut trader_token_state_maybe = MaybeUninit::<TraderTokenState>::uninit();
    let trader_token_state = unsafe { TraderTokenState::load(key, &mut trader_token_state_maybe) };
    trader_token_state.lots_free += params.lots;

    unsafe {
        trader_token_state.store(key);
        storage_flush_cache(true);
    }

    0
}

#[cfg(test)]
mod test {
    use super::*;

    use hex_literal::hex;

    use crate::{
        getter::read_trader_token_state,
        hostio::*,
        quantities::Lots,
        state::{TraderTokenKey, TraderTokenState},
        user_entrypoint,
    };

    use super::{CreditERC20Params, HANDLE_1_CREDIT_ERC20};

    #[test]
    pub fn test_deposit_erc20() {
        // Set hostios
        let mut msg_sender = [0u8; 32];
        msg_sender[12..].copy_from_slice(&hex!("3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E"));
        set_msg_sender(msg_sender);

        let mut return_data = vec![0u8; 32];
        return_data[31] = 1;
        set_return_data(return_data);

        // Set args
        let mut test_args: Vec<u8> = vec![];
        let num_calls: u8 = 1;
        test_args.push(num_calls);
        test_args.push(HANDLE_1_CREDIT_ERC20);

        let payload_len = core::mem::size_of::<CreditERC20Params>();
        test_args.push(payload_len as u8);

        let payload = CreditERC20Params {
            token: hex!("7E32b54800705876d3b5cFbc7d9c226a211F7C1a"),
            recipient: hex!("3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E"),
            lots: Lots(1),
        };

        // Serialize into bytes array
        let payload_bytes: &[u8] = unsafe {
            core::slice::from_raw_parts(
                &payload as *const CreditERC20Params as *const u8,
                core::mem::size_of::<CreditERC20Params>(),
            )
        };
        test_args.extend_from_slice(payload_bytes);
        set_test_args(test_args.clone());

        let result = user_entrypoint(test_args.len());
        assert_eq!(result, 0);

        let key = &TraderTokenKey {
            trader: payload.recipient,
            token: payload.token,
        };

        let mut trader_token_state_maybe = MaybeUninit::<TraderTokenState>::uninit();
        let trader_token_state =
            unsafe { TraderTokenState::load(key, &mut trader_token_state_maybe) };

        assert_eq!(trader_token_state.lots_free.0, 1);
        assert_eq!(trader_token_state.lots_locked.0, 0);

        // Validate result from getter
        let trader_token_state_bytes = read_trader_token_state(key);
        let trader_token_state: &TraderTokenState =
            unsafe { &*(trader_token_state_bytes.as_ptr() as *const TraderTokenState) };

        assert_eq!(trader_token_state.lots_free.0, 1);
        assert_eq!(trader_token_state.lots_locked.0, 0);
    }
}
