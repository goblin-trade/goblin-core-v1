// Low-level Stylus hostio bindings and the native "test hostio" emulation live
// in the `goblin-hostio` crate. Re-export the module so existing
// `crate::hostio::hostio_unsafe` call sites keep working.
pub use goblin_hostio::hostio_unsafe;

pub mod erc20_hostio;
pub mod eth_hostio;
pub mod hostio_helpers;

pub use erc20_hostio::*;
pub use eth_hostio::*;
pub use hostio_helpers::*;

#[cfg(all(not(test), not(target_arch = "wasm32")))]
pub mod indexer_hostio;
#[cfg(all(not(test), not(target_arch = "wasm32")))]
pub use indexer_hostio::*;
