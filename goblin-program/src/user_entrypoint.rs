use goblin_core::{
    goblin_error::GoblinError,
    input_processor::{ArgsBufferV2, ArgsReaderV2, GlobalArgs},
    require,
    settlement::StaticDelta,
};
use goblin_hostio::hostio_helpers;

#[unsafe(no_mangle)]
pub extern "C" fn user_entrypoint(_len: usize) -> i32 {
    match user_entrypoint_inner() {
        Ok(_) => 0,
        Err(err) => err.code(),
    }
}

fn user_entrypoint_inner() -> Result<(), GoblinError> {
    require!(!hostio_helpers::msg_reentrant(), GoblinError::Reentrant);

    let delta = StaticDelta::get();

    let args_buffer = ArgsBufferV2::default();
    let mut reader: ArgsReaderV2 = (&args_buffer).into();

    let global_args = GlobalArgs::new(&mut reader)?;
    global_args.process(&mut reader, delta)?;

    // Write cache to trie
    // https://github.com/OffchainLabs/stylus-sdk-rs/blob/2c709a5a1a620ed7585c7d8af64fefabe3a0fc9a/stylus-sdk/src/storage/mod.rs#L81
    hostio_helpers::storage_flush_cache(false);

    Ok(())
}
