pub mod debug_hooks;

// Real Stylus hostio imports (the contract). Excluded when the native "test
// hostio" emulation below is in use: either this crate's own unit tests, or a
// dependent crate that enables the `test-hostio` feature.
#[cfg(not(any(test, feature = "test-hostio")))]
pub mod vm_hooks;
#[cfg(not(any(test, feature = "test-hostio")))]
pub use vm_hooks::*;

#[cfg(any(test, feature = "test-hostio"))]
mod test_suite;
#[cfg(any(test, feature = "test-hostio"))]
pub use test_suite::*;
