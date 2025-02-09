#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

pub mod handler;
pub mod hostio;
pub mod market_params;
pub mod quantities;
pub mod selector;
// extern crate alloc;
// use alloc::vec::Vec;
use handler::handle_deposit_funds;
use hostio::*;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// #[cfg(target_arch = "wasm32")]
// #[global_allocator]
// static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

#[no_mangle]
pub unsafe extern "C" fn mark_used() {
    pay_for_memory_grow(0);
    panic!();
}

#[no_mangle]
pub extern "C" fn user_entrypoint(len: usize) -> i32 {
    if len == 0 || len > 90 {
        return 1;
    }

    // Use fixed-size array on the stack instead of heap allocation
    let mut input = [0u8; 90];

    unsafe {
        read_args(input.as_mut_ptr());
    }

    // Extract function selector
    let selector = input[0];
    let payload = &input[1..len]; // Only use the actual input length

    // Route to appropriate handler based on selector
    return match selector {
        selector::DEPOSIT_FUNDS_SELECTOR => handle_deposit_funds(payload),
        _ => 1,
    };
}
