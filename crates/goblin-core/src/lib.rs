#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

pub mod handler;
pub mod hostio;
pub mod market_params;
pub mod quantities;
pub mod selector;
extern crate alloc;
use alloc::vec::Vec;
use hostio::*;
use market_params::MarketParams;

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
    if len == 0 {
        return 1;
    }

    // The input length is known from `len`
    // read_args will read the input at the pointer location
    let mut input = Vec::<u8>::with_capacity(len);

    unsafe {
        // set_len() is necessary
        input.set_len(len);
        read_args(input.as_mut_ptr());
    }

    // Extract function selector
    let selector = input[0];
    let payload = &input[1..];

    // Route to appropriate handler based on selector
    return match selector {
        selector::DEPOSIT_FUNDS_SELECTOR => {
            let deposit_msg = "Depositing funds";
            unsafe {
                log_txt(deposit_msg.as_ptr(), deposit_msg.len());
            }

            if payload.len() < core::mem::size_of::<MarketParams>() {
                return 1;
            }
            let market_params = unsafe { &*(payload.as_ptr() as *const MarketParams) };

            #[cfg(test)]
            println!("got market params {:?}", *market_params);

            unsafe {
                log_i64(market_params.base_lot_size.0 as i64);
            }
            0
        }
        _ => 1,
    };
}
