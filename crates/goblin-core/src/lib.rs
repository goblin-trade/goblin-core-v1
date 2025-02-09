#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use core::mem::MaybeUninit;
use handler::handle_deposit_funds;
use hostio::*;

pub mod handler;
pub mod hostio;
pub mod market_params;
pub mod quantities;
pub mod selector;

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

// Static buffer for input
static mut INPUT_BUFFER: [u8; 512] = [0; 512];

#[no_mangle]
pub extern "C" fn user_entrypoint(len: usize) -> i32 {
    if len == 0 {
        return 1;
    }

    let (selector, payload) = unsafe {
        read_args(INPUT_BUFFER.as_mut_ptr());

        (
            INPUT_BUFFER[0],
            core::slice::from_raw_parts(INPUT_BUFFER.as_ptr().add(1), len.saturating_sub(1)),
        )
    };

    match selector {
        selector::DEPOSIT_FUNDS_SELECTOR => handle_deposit_funds(payload),
        _ => 1,
    }
}
