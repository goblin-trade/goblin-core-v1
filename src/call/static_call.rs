use crate::hostio;

pub unsafe fn static_call(
    contract: *const u8,
    calldata: *const u8,
    calldata_len: usize,
    gas: u64,
    return_data_len: *mut usize,
) -> u8 {
    // Flush storage to persist changes but don't invalidate cache
    // https://github.com/OffchainLabs/stylus-sdk-rs/blob/cb9d2ba9877f66d8d65d61b45e340cbbc6750593/stylus-sdk/src/call/mod.rs#L58
    hostio::storage_flush_cache(false);
    hostio::static_call_contract(contract, calldata, calldata_len, gas, return_data_len)
}
