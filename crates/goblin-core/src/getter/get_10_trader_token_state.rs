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

#[cfg(test)]
pub fn read_trader_token_state(trader_token_key: &TraderTokenKey) -> Vec<u8> {
    use crate::user_entrypoint;

    let mut test_args: Vec<u8> = vec![];
    let num_calls: u8 = 1;
    test_args.push(num_calls);
    test_args.push(GET_10_TRADER_TOKEN_STATE);

    let payload_len = core::mem::size_of::<TraderTokenKey>();
    test_args.push(payload_len as u8);

    let payload_bytes: &[u8] = unsafe {
        core::slice::from_raw_parts(
            trader_token_key as *const TraderTokenKey as *const u8,
            core::mem::size_of::<TraderTokenKey>(),
        )
    };
    test_args.extend_from_slice(payload_bytes);
    crate::set_test_args(test_args.clone());
    user_entrypoint(test_args.len());

    let result_vec = crate::get_test_result();
    result_vec
}

#[cfg(test)]
mod test {
    use hex_literal::hex;

    use super::*;

    #[test]
    fn test_read_default_trader_token_state() {
        let key = TraderTokenKey {
            token: hex!("7E32b54800705876d3b5cFbc7d9c226a211F7C1a"),
            trader: hex!("3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E"),
        };

        let result_vec = read_trader_token_state(&key);
        let trader_token_state: &TraderTokenState =
            unsafe { &*(result_vec.as_ptr() as *const TraderTokenState) };

        assert_eq!(trader_token_state.lots_free.0, 0);
        assert_eq!(trader_token_state.lots_locked.0, 0);
    }
}
