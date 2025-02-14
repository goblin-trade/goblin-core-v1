use core::mem::MaybeUninit;

use crate::{
    msg_value,
    quantities::Lots,
    state::{SlotState, TraderTokenKey, TraderTokenState},
    storage_flush_cache,
    types::{Address, NATIVE_TOKEN},
};
pub const HANDLE_0_CREDIT_ETH: u8 = 0;

/// Credit ETH to a recipient
pub fn handle_0_credit_eth(payload: &[u8]) -> i32 {
    if payload.len() != 20 {
        return 1;
    }

    let recipient = unsafe { &*(payload.as_ptr() as *const Address) };

    // Amount of ETH in, in 64-bit chunks
    let mut amount_in_maybe = MaybeUninit::<[u64; 4]>::uninit();
    let amount_in = unsafe {
        msg_value(amount_in_maybe.as_mut_ptr() as *mut u8);
        amount_in_maybe.assume_init_ref()
    };
    let lots = Lots::from_atoms(amount_in);

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

    use crate::{set_msg_value, set_test_args, user_entrypoint};

    use super::HANDLE_0_CREDIT_ETH;

    #[test]
    pub fn test_deposit() {
        // Set msg.value to 10^6
        let msg_value = hex!("00000000000000000000000000000000000000000000000000000000000F4240");
        set_msg_value(msg_value);

        // Set args
        let mut test_args: Vec<u8> = vec![];
        test_args.push(HANDLE_0_CREDIT_ETH);
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
    }
}
