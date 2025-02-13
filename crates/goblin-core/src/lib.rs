#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use core::mem::MaybeUninit;
use handler::{handle_0_credit_eth, HANDLE_0_CREDIT_ETH};
use hostio::*;

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

    let (selector, payload) = unsafe {
        let mut input = MaybeUninit::<[u8; 512]>::uninit();
        read_args(input.as_mut_ptr() as *mut u8);
        let input = input.assume_init_ref();

        (
            input[0],
            core::slice::from_raw_parts(&input[1], len.saturating_sub(1)),
        )
    };

    match selector {
        HANDLE_0_CREDIT_ETH => handle_0_credit_eth(payload),
        _ => 1,
    }
}
