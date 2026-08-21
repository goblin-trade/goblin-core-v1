#![allow(static_mut_refs)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use crate::processor::processor;

pub mod axis;
pub mod axis_helpers;
pub mod ctx;
pub mod goblin_error;
pub mod hostio;
pub mod input_processor;
pub mod instructions;
pub mod market;
pub mod matching;
pub mod processor;
pub mod quantities;
pub mod settlement;
pub mod state;
pub mod types;

pub use ctx::*;

#[no_mangle]
pub extern "C" fn user_entrypoint(len: usize) -> i32 {
    match processor(len) {
        Ok(_) => 0,
        Err(err) => err.code(),
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(all(not(test), target_arch = "wasm32"))]
#[no_mangle]
pub unsafe extern "C" fn mark_used() {
    crate::hostio::hostio_unsafe::pay_for_memory_grow(0);
    panic!();
}
