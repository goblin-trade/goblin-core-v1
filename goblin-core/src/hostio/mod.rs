// Low-level Stylus hostio bindings and the native "test hostio" emulation live
// in the `goblin-hostio` crate. Re-export the module so existing
// `crate::hostio::hostio_unsafe` call sites keep working.
pub use goblin_hostio::hostio_helpers;
pub use goblin_hostio::hostio_unsafe;

// Safe contract-call helpers stay in goblin-core because they build on core
// types (Address, RawAtoms, GoblinError, ...) rather than hostios alone.
pub mod call_helpers;

pub mod erc20_hostio;
pub mod eth_hostio;

pub use call_helpers::*;
pub use erc20_hostio::*;
pub use eth_hostio::*;
// Pure hostio wrappers moved to the goblin-hostio crate. Re-export them so
// existing `crate::hostio::msg_sender()`-style call sites keep working.
pub use goblin_hostio::hostio_helpers::{
    block_number, block_timestamp, msg_reentrant, msg_sender, msg_value, native_keccak256,
    storage_cache_bytes32, storage_flush_cache, storage_load_bytes32,
};

#[cfg(all(not(test), not(target_arch = "wasm32")))]
pub mod indexer_hostio;
#[cfg(all(not(test), not(target_arch = "wasm32")))]
pub use indexer_hostio::*;
