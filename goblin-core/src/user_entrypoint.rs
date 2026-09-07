use crate::{
    goblin_error::GoblinError,
    hostio,
    input_processor::{ArgsReader, CompoundDecode, GlobalArgs},
    require,
    settlement::StaticDelta,
};
use hex_literal::hex;

pub const CONTRACT_ADDRESS: [u8; 20] = hex!("8888ef09a63b6328468fce63a09fc185de807722");

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
