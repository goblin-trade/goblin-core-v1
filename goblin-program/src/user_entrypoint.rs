use core::mem::MaybeUninit;

use deku::{
    DekuContainerRead, DekuReader, DekuWriter, no_std_io::Cursor, reader::Reader, writer::Writer,
};
use goblin_core::{
    goblin_error::GoblinError,
    input_processor::{
        ArgsReader, CompoundDecode, GlobalArgs, HeaderFlags, INPUT_SIZE, header_flags,
    },
    require,
    settlement::StaticDelta,
};
use goblin_hostio::{hostio_helpers, hostio_unsafe};

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

    // let mut args_buffer = MaybeUninit::<[u8; INPUT_SIZE]>::uninit();
    // let args = unsafe {
    //     hostio_unsafe::read_args(args_buffer.as_mut_ptr() as *mut u8);
    //     args_buffer.assume_init_mut()
    // };
    // let cursor = Cursor::new(args.as_mut_slice());
    // let deku_reader = &mut Reader::new(cursor);

    // let header_flags = HeaderFlags::from_reader_with_ctx(deku_reader, ());

    // let deku_writer = &mut Writer::new(cursor);

    // let header_flags = HeaderFlags::default();
    // header_flags
    //     .to_writer(deku_writer, ())
    //     .map_err(|_| GoblinError::CallFail)?;

    // let global_args = GlobalArgs::try_compound_decode(reader)?;
    // global_args.process(reader, delta)?;

    // Write cache to trie
    // https://github.com/OffchainLabs/stylus-sdk-rs/blob/2c709a5a1a620ed7585c7d8af64fefabe3a0fc9a/stylus-sdk/src/storage/mod.rs#L81
    hostio_helpers::storage_flush_cache(false);

    Ok(())
}
