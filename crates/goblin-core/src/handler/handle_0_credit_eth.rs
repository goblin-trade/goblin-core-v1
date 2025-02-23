use core::mem::MaybeUninit;

use crate::{
    msg_value,
    quantities::{Atoms, Lots},
    state::{SlotState, TraderTokenKey, TraderTokenState},
    storage_flush_cache,
    types::{Address, NATIVE_TOKEN},
};
pub const HANDLE_0_CREDIT_ETH: u8 = 0;

/// Credit ETH to a recipient
///
/// * Wei is passed using `--value` and read with `msg_value`. It is big endian encoded.
///
/// * The address is encoded in `payload`. The client call encodes the data such that we obtain
/// the big endian result in a slice without need of any processing.
///
/// # Example
///
/// ```
/// cast send 0xa6e41ffd769491a42a6e5ce453259b93983a22ef \
///   0x003f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E \
///   --value 1000000wei \
///   --rpc-url http://127.0.0.1:8547 \
///   --private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520
/// ```
///
/// * After removing selector `00` we're left with payload `3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E`
/// * This payload is decoded as [0x3f, 0x1E, ..., 0E]
/// * The address is already in big endian
///
pub fn handle_0_credit_eth(payload: &[u8]) -> i32 {
    if payload.len() != 20 {
        return 1;
    }

    let recipient: &Address = unsafe { &*(payload.as_ptr() as *const Address) };

    // Amount of ETH in, in 64-bit chunks, in big endian encoding
    let mut amount_in_maybe = MaybeUninit::<Atoms>::uninit();
    let amount_in = unsafe {
        msg_value(amount_in_maybe.as_mut_ptr() as *mut u8);
        amount_in_maybe.assume_init_ref()
    };
    let lots = Lots::from(amount_in);

    let key = &TraderTokenKey {
        trader: *recipient,
        token: NATIVE_TOKEN,
    };

    let mut trader_token_state_maybe = MaybeUninit::<TraderTokenState>::uninit();
    let trader_token_state = unsafe { TraderTokenState::load(key, &mut trader_token_state_maybe) };
    trader_token_state.lots_free += lots;

    unsafe {
        trader_token_state.store(key);
        storage_flush_cache(true);
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    use crate::{getter::read_trader_token_state, set_msg_value, set_test_args, user_entrypoint};

    use super::HANDLE_0_CREDIT_ETH;

    #[test]
    pub fn test_deposit() {
        // Set msg.value to 10^6 in big endian
        let msg_value = hex!("00000000000000000000000000000000000000000000000000000000000F4240");
        set_msg_value(msg_value);

        // Set args
        let mut test_args: Vec<u8> = vec![];
        let num_calls: u8 = 1;
        test_args.push(num_calls);
        test_args.push(HANDLE_0_CREDIT_ETH);

        let payload_len = 20; // 20 byte address
        test_args.push(payload_len);

        let recipient = hex!("3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E");
        test_args.extend_from_slice(&recipient);
        set_test_args(test_args.clone());

        let result = user_entrypoint(test_args.len());
        assert_eq!(result, 0);

        // Check lot balance
        let key = &TraderTokenKey {
            trader: recipient,
            token: NATIVE_TOKEN,
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
