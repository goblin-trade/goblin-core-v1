#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

pub mod handler;
pub mod hostio;
pub mod market_params;
pub mod selector;
extern crate alloc;
use alloc::vec::Vec;
use handler::{handle_get_count, handle_set_count};
use hostio::*;
use market_params::MarketParams;
use selector::{GET_COUNT_SELECTOR, SET_COUNT_SELECTOR};

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

// Storage key for the counter value
pub const COUNTER_KEY: [u8; 32] = [0; 32];

#[no_mangle]
pub unsafe extern "C" fn mark_used() {
    pay_for_memory_grow(0);
    panic!();
}

#[no_mangle]
pub extern "C" fn user_entrypoint(len: usize) -> i32 {
    let mut input = Vec::<u8>::with_capacity(len);

    unsafe {
        input.set_len(len);
        read_args(input.as_mut_ptr());
    }

    // Check for minimum length for selector
    if input.len() < 4 {
        return 1;
    }

    // Extract function selector
    let selector = &input[0..4];
    let payload = &input[4..];

    // Route to appropriate handler based on selector
    return match selector {
        sel if sel == selector::DEPOSIT_FUNDS_SELECTOR => {
            if payload.len() < core::mem::size_of::<MarketParams>() {
                return 1;
            }
            let market_params = unsafe { &*(payload.as_ptr() as *const MarketParams) };

            0
        }
        _ => {
            return 1; // Unknown selector
        }
    };
}
