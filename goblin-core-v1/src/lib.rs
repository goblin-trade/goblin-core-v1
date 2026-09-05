#![allow(static_mut_refs)]
#![cfg_attr(all(not(any(test, feature = "sdk")), target_arch = "wasm32"), no_std)]
#![cfg_attr(all(not(any(test, feature = "sdk")), target_arch = "wasm32"), no_main)]

mod mods;
pub use mods::*;

use crate::processor::processor;

#[no_mangle]
pub extern "C" fn user_entrypoint(len: usize) -> i32 {
    match processor(len) {
        Ok(_) => 0,
        Err(err) => err.code(),
    }
}

#[cfg(all(not(any(test, feature = "sdk")), target_arch = "wasm32"))]
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
