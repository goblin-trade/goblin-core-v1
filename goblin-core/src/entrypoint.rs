//! The Stylus contract entrypoint.
//!
//! This is the orchestration layer that the wasm ABI shim in `goblin-program`
//! calls. It lives here, rather than in `goblin-program`, so it can be exercised
//! directly by tests: it only relies on `goblin-core`'s own pieces
//! ([`ArgsBuffer`], [`GlobalInput`], [`StaticDelta`]) plus the `goblin-hostio`
//! helpers.

use goblin_hostio::hostio_helpers;

use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, ArgsReader, GlobalInput},
    require,
    settlement::StaticDelta,
};

/// Decode the call's arguments, process them, and flush the storage cache.
///
/// Returns the [`GoblinError`] (and thus the revert code) on failure.
pub fn entrypoint() -> Result<(), GoblinError> {
    require!(!hostio_helpers::msg_reentrant(), GoblinError::Reentrant);

    let delta = StaticDelta::get();

    let args_buffer = ArgsBuffer::default();
    let reader = &mut ArgsReader::from(&args_buffer);

    let global_input = GlobalInput::new(reader)?;
    global_input.process(reader, delta)?;

    // Write cache to trie
    // https://github.com/OffchainLabs/stylus-sdk-rs/blob/2c709a5a1a620ed7585c7d8af64fefabe3a0fc9a/stylus-sdk/src/storage/mod.rs#L81
    hostio_helpers::storage_flush_cache(false);

    Ok(())
}
