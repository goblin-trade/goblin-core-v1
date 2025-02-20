#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use core::mem::MaybeUninit;
use getter::{get_10_trader_token_state, GET_10_TRADER_TOKEN_STATE};
use handler::{
    handle_0_credit_eth, handle_1_credit_erc20, HANDLE_0_CREDIT_ETH, HANDLE_1_CREDIT_ERC20,
};
use hostio::*;

pub mod erc20;
pub mod getter;
pub mod handler;
pub mod hostio;
pub mod market_params;
pub mod quantities;
pub mod state;
pub mod types;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn mark_used() {
    pay_for_memory_grow(0);
    panic!();
}

#[no_mangle]
pub extern "C" fn user_entrypoint(len: usize) -> i32 {
    if len == 0 {
        return 1;
    }

    let mut input = MaybeUninit::<[u8; 512]>::uninit();
    let (selector, payload) = unsafe {
        read_args(input.as_mut_ptr() as *mut u8);
        let input = input.assume_init_ref();

        (
            input[0],
            core::slice::from_raw_parts(&input[1], len.saturating_sub(1)),
        )
    };

    // Equals [166, ..., 239]. This is correct.
    // Unsafe cast in handle_1_credit_erc20() breaks the address
    unsafe {
        let msg = b"Payload byte 0";
        log_txt(msg.as_ptr(), msg.len());
        log_i64(payload[0] as i64);

        let msg = b"Payload byte 19";
        log_txt(msg.as_ptr(), msg.len());
        log_i64(payload[19] as i64);
    }

    match selector {
        HANDLE_0_CREDIT_ETH => handle_0_credit_eth(payload),
        HANDLE_1_CREDIT_ERC20 => handle_1_credit_erc20(payload),

        // Getters
        GET_10_TRADER_TOKEN_STATE => get_10_trader_token_state(payload),

        _ => 1,
    }
}
