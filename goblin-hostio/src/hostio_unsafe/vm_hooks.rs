// VM hooks used for hostio
//
// # Note on re-entry and flushing
//
// Since re-entrancy is is disabled, we only need to flush once before exiting.
// Ref- https://github.com/OffchainLabs/stylus-sdk-rs/blob/2c709a5a1a620ed7585c7d8af64fefabe3a0fc9a/stylus-sdk/src/storage/mod.rs#L81
//
// If re-entrancy is enabled
//
// 1. Execute storage_flush_cache(false) before making call().
// Ref- https://github.com/OffchainLabs/stylus-sdk-rs/blob/2c709a5a1a620ed7585c7d8af64fefabe3a0fc9a/stylus-sdk/src/call/mod.rs#L73
//
// 2. Execute storage_flush_cache(true) before making static_call() and delegate_call().
// Ref- https://github.com/OffchainLabs/stylus-sdk-rs/blob/2c709a5a1a620ed7585c7d8af64fefabe3a0fc9a/stylus-sdk/src/call/mod.rs#L36
//
#[cfg_attr(target_arch = "wasm32", link(wasm_import_module = "vm_hooks"))]
extern "C" {
    pub fn read_args(dest: *mut u8);
    pub fn write_result(data: *const u8, len: usize);
    pub fn pay_for_memory_grow(pages: u16);
    pub fn storage_load_bytes32(key: *const u8, dest: *mut u8);
    pub fn storage_cache_bytes32(key: *const u8, value: *const u8);
    pub fn storage_flush_cache(clear: bool);
    pub fn native_keccak256(bytes: *const u8, len: usize, output: *mut u8);
    pub fn msg_value(value: *mut u8);
    pub fn msg_sender(sender: *mut u8);
    pub fn block_number() -> u64;
    pub fn block_timestamp() -> u64;
    pub fn call_contract(
        contract: *const u8,
        calldata: *const u8,
        calldata_len: usize,
        value: *const u8,
        gas: u64,
        return_data_len: *mut usize,
    ) -> u8;
    pub fn static_call_contract(
        contract: *const u8,
        calldata: *const u8,
        calldata_len: usize,
        gas: u64,
        return_data_len: *mut usize,
    ) -> u8;
    pub fn read_return_data(dest: *mut u8, offset: usize, size: usize) -> usize;
    pub fn msg_reentrant() -> bool;
}
