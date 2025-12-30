pub mod debug_hooks;

#[cfg(not(test))]
pub mod vm_hooks;
#[cfg(not(test))]
pub use vm_hooks::*;

#[cfg(test)]
mod tests;

#[cfg(test)]
pub use tests::*;
