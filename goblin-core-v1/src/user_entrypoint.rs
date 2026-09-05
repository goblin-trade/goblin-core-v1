use crate::{
    goblin_error::GoblinError,
    hostio,
    input_processor::{ArgsReader, CompoundDecode, GlobalArgs},
    require,
    settlement::StaticDelta,
};

pub const CONTRACT_ADDRESS: [u8; 20] = [
    0x88, 0x88, 0xef, 0x09, 0xa6, 0x3b, 0x63, 0x28, 0x46, 0x8f, 0xce, 0x63, 0xa0, 0x9f, 0xc1, 0x85,
    0xde, 0x80, 0x77, 0x22,
];

#[no_mangle]
pub extern "C" fn user_entrypoint(len: usize) -> i32 {
    match user_entrypoint_inner(len) {
        Ok(_) => 0,
        Err(err) => err.code(),
    }
}

fn user_entrypoint_inner(len: usize) -> Result<(), GoblinError> {
    require!(!hostio::msg_reentrant(), GoblinError::Reentrant);

    let delta = StaticDelta::get();
    let reader = &mut ArgsReader::new(len);

    let global_args = GlobalArgs::try_compound_decode(reader)?;
    global_args.process(reader, delta)?;

    // Write cache to trie
    // https://github.com/OffchainLabs/stylus-sdk-rs/blob/2c709a5a1a620ed7585c7d8af64fefabe3a0fc9a/stylus-sdk/src/storage/mod.rs#L81
    hostio::storage_flush_cache(false);

    Ok(())
}
