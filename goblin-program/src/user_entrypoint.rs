use goblin_core::{
    goblin_error::GoblinError,
    input_processor::{ArgsReader, CompoundDecode, GlobalArgs},
    require,
    settlement::StaticDelta,
};
use goblin_hostio::hostio_helpers;

#[unsafe(no_mangle)]
pub extern "C" fn user_entrypoint(len: usize) -> i32 {
    match user_entrypoint_inner(len) {
        Ok(_) => 0,
        Err(err) => err.code(),
    }
}

fn user_entrypoint_inner(len: usize) -> Result<(), GoblinError> {
    require!(!hostio_helpers::msg_reentrant(), GoblinError::Reentrant);

    let delta = StaticDelta::get();
    let reader = &mut ArgsReader::new(len);

    let global_args = GlobalArgs::try_compound_decode(reader)?;
    global_args.process(reader, delta)?;

    // Write cache to trie
    // https://github.com/OffchainLabs/stylus-sdk-rs/blob/2c709a5a1a620ed7585c7d8af64fefabe3a0fc9a/stylus-sdk/src/storage/mod.rs#L81
    hostio_helpers::storage_flush_cache(false);

    Ok(())
}
