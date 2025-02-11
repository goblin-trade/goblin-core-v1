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

#[no_mangle]
pub extern "C" fn user_entrypoint(len: usize) -> i32 {
    if len == 0 {
        return 1;
    }

    let (selector, payload) = unsafe {
        let mut input: [MaybeUninit<u8>; 512] = MaybeUninit::uninit().assume_init();
        read_args(input.as_mut_ptr() as *mut u8);

        (
            input[0].assume_init(),
            core::slice::from_raw_parts(input.as_ptr().add(1) as *const u8, len.saturating_sub(1)),
        )
    };

    match selector {
        selector::DEPOSIT_FUNDS_SELECTOR => handle_deposit_funds(payload),
        _ => 1,
    }
}
