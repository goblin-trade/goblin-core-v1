use core::mem::MaybeUninit;

use crate::{
    log_i64, log_txt, msg_value,
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
        let msg = b"New free lots";
        log_txt(msg.as_ptr(), msg.len());
        log_i64(trader_token_state.lots_free.0 as i64);

        trader_token_state.store(key);
        storage_flush_cache(true);
    }

    0
}
