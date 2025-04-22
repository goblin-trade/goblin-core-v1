use crate::hostio;

pub unsafe fn clear_cache_and_call(
    contract: *const u8,
    calldata: *const u8,
    calldata_len: usize,
    value: *const u8,
    gas: u64,
    return_data_len: *mut usize,
) -> u8 {
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
