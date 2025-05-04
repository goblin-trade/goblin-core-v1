use crate::hostio;

pub unsafe fn clear_cache_and_call(
    contract: *const u8,
    calldata: *const u8,
    calldata_len: usize,
    value: *const u8,
    gas: u64,
    return_data_len: *mut usize,
) -> u8 {
    // Flush storage to persist changes, then invalidate the cache
    // https://github.com/OffchainLabs/stylus-sdk-rs/blob/cb9d2ba9877f66d8d65d61b45e340cbbc6750593/stylus-sdk/src/call/mod.rs#L102
    hostio::storage_flush_cache(true);
    hostio::call_contract(
        contract,
        calldata,
        calldata_len,
        value,
        gas,
        return_data_len,
    )
}
