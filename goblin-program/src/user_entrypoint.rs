//! Wasm ABI shim. The contract logic lives in [`goblin_core::entrypoint`]; this
//! only adapts it to the `extern "C"` signature Stylus expects.

use goblin_core::entrypoint;

#[unsafe(no_mangle)]
pub extern "C" fn user_entrypoint(_len: usize) -> i32 {
    match entrypoint() {
        Ok(()) => 0,
        Err(err) => err.code(),
    }
}
