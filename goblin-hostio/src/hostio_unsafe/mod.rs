pub mod debug_hooks;

// Real Stylus hostio imports (the contract). Excluded when running tests so the
// "test hostio" emulation below is used instead.
#[cfg(not(test))]
pub mod vm_hooks;
#[cfg(not(test))]
pub use vm_hooks::*;

#[cfg(test)]
mod tests;
#[cfg(test)]
pub use tests::*;
