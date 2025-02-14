use core::mem::MaybeUninit;

use crate::{
    state::{SlotState, TraderTokenKey, TraderTokenState},
    write_result,
};
pub const GET_10_TRADER_TOKEN_STATE: u8 = 10;

pub fn get_10_trader_token_state(payload: &[u8]) -> i32 {
    if payload.len() != core::mem::size_of::<TraderTokenKey>() {
        return 1;
    }

    let trader_token_key = unsafe { &*(payload.as_ptr() as *const TraderTokenKey) };

    let mut trader_token_state_maybe = MaybeUninit::<TraderTokenState>::uninit();

    unsafe {
        let trader_token_state =
            TraderTokenState::load(trader_token_key, &mut trader_token_state_maybe);

        write_result(
            trader_token_state as *const TraderTokenState as *const u8,
            core::mem::size_of::<TraderTokenState>(),
        );
    }

    0
}
