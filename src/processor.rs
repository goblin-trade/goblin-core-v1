use crate::{
    axis::market::market_counts::MarketCounts,
    goblin_error::GoblinError,
    hostio::{self},
    input_processor::{DecodeCtx, GlobalHeader},
    require,
    settlement::Delta,
};

pub const CONTRACT_ADDRESS: [u8; 20] = [
    0x88, 0x88, 0x41, 0x5d, 0xb8, 0x0e, 0xab, 0xcf, 0x58, 0x02, 0x83, 0xa3, 0xd6, 0x52, 0x49, 0x88,
    0x7d, 0x31, 0x61, 0xb0,
];

pub fn processor(len: usize) -> Result<(), GoblinError> {
    let msg_reentrant = hostio::msg_reentrant();
    require!(!msg_reentrant, GoblinError::Reentrant);

    let msg_sender = hostio::msg_sender();
    let delta = Delta::get_static();
    let ctx = &mut DecodeCtx::new(len);

    let global_header = GlobalHeader::new(ctx)?;
    global_header.process(ctx, &msg_sender, delta)?;

    // Write cache to trie
    // https://github.com/OffchainLabs/stylus-sdk-rs/blob/2c709a5a1a620ed7585c7d8af64fefabe3a0fc9a/stylus-sdk/src/storage/mod.rs#L81
    hostio::storage_flush_cache(false);

    Ok(())
}
