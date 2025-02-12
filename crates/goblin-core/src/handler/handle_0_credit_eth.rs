use crate::{
    msg_value,
    state::{SlotState, TraderTokenKey, TraderTokenState},
    storage_load_bytes32,
    types::{Address, NATIVE_TOKEN},
};
pub const HANDLE_0_CREDIT_ETH: u8 = 0;

/// Credit ETH to a recipient
pub fn handle_0_credit_eth(payload: &[u8]) -> i32 {
    if payload.len() != 20 {
        return 1;
    }
    let recipient = unsafe { &*(payload.as_ptr() as *const Address) };

    let mut amount_in = [0u8; 32];
    unsafe {
        msg_value(amount_in.as_mut_ptr());
    }

    // General flow
    // Find key

    let trader_token_state = TraderTokenState::load(&TraderTokenKey {
        trader: *recipient,
        token: NATIVE_TOKEN,
    });

    0
}
