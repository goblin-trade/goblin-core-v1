use crate::{
    log_i64, log_txt, msg_value,
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
    let mut amount_in = [0u64; 4];
    unsafe {
        msg_value(amount_in.as_mut_ptr() as *mut u8);
    }
    // The bytes are in big endian format. However when we view it as u64, it is little endian.
    // We need to reverse the bytes to get the correct value.
    let high = amount_in[2].swap_bytes();
    let low = amount_in[3].swap_bytes();

    unsafe {
        let msg = b"High and low";
        log_txt(msg.as_ptr(), msg.len());
        log_i64(high as i64);
        log_i64(low as i64);
    }

    const SCALE: u64 = 18446744073709; // (2^64 / 10^6)
    let high_lots = high.wrapping_mul(SCALE);

    // For low bits, direct division is fine
    let low_lots = low / 1_000_000;

    let lots = high_lots + low_lots;
    unsafe {
        let msg = b"lots";
        log_txt(msg.as_ptr(), msg.len());
        log_i64(lots as i64);
    }

    let key = &TraderTokenKey {
        trader: *recipient,
        token: NATIVE_TOKEN,
    };

    let state = TraderTokenState::load(key);
    unsafe {
        let msg = b"Locked and free lots read";
        log_txt(msg.as_ptr(), msg.len());
        log_i64(state.lots_locked as i64);
        log_i64(state.lots_free as i64);
    }

    state.lots_free += lots;
    state.store(key);

    unsafe {
        storage_flush_cache(true);
    }

    0
}
