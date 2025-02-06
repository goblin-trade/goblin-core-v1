#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

extern crate alloc;
use alloc::vec::Vec;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

// VM hooks
#[link(wasm_import_module = "vm_hooks")]
extern "C" {
    fn read_args(dest: *mut u8);
    fn write_result(data: *const u8, len: usize);
    fn pay_for_memory_grow(pages: u16);
    fn storage_load_bytes32(key: *const u8, dest: *mut u8);
    fn storage_cache_bytes32(key: *const u8, value: *const u8);
    fn storage_flush_cache(clear: bool);
}

#[link(wasm_import_module = "console")]
extern "C" {
    fn log_txt(text: *const u8, len: usize);
}

// Constants for function selectors
// Find hash from https://emn178.github.io/online-tools/keccak_256.html
const SET_COUNT_SELECTOR: [u8; 4] = [0xd1, 0x4e, 0x62, 0xb8]; // keccak256("setCount(uint256)")[:4] = d14e62b8
const GET_COUNT_SELECTOR: [u8; 4] = [0xa8, 0x7d, 0x94, 0x2c]; // keccak256("getCount()")[:4] = a87d942c

// Storage key for the counter value
const COUNTER_KEY: [u8; 32] = [0; 32];

// Helper function to read counter value from storage
fn read_counter() -> u64 {
    let mut value = [0u8; 32];
    unsafe {
        storage_load_bytes32(COUNTER_KEY.as_ptr(), value.as_mut_ptr());
    }
    u64::from_be_bytes(value[24..32].try_into().unwrap())
}

// Helper function to write counter value to storage
fn write_counter(value: u64) {
    let mut bytes = [0u8; 32];
    bytes[24..32].copy_from_slice(&value.to_be_bytes());
    unsafe {
        storage_cache_bytes32(COUNTER_KEY.as_ptr(), bytes.as_ptr());
        storage_flush_cache(true);
    }
}

// Function to handle setCount(uint256)
fn handle_set_count(input: &[u8]) {
    if input.len() != 32 {
        return;
    }

    // Read the uint256 parameter (we'll only use the last 8 bytes for u64)
    let value = u64::from_be_bytes(input[24..32].try_into().unwrap());
    write_counter(value);
}

// Function to handle getCount()
fn handle_get_count() -> Vec<u8> {
    let value = read_counter();
    let mut result = Vec::with_capacity(32);
    result.extend_from_slice(&[0u8; 24]); // Pad with zeros
    result.extend_from_slice(&value.to_be_bytes()); // Append the u64 value
    result
}

#[no_mangle]
pub unsafe extern "C" fn mark_used() {
    pay_for_memory_grow(0);
    panic!();
}

#[no_mangle]
pub extern "C" fn user_entrypoint(len: usize) -> i32 {
    let mut input = Vec::<u8>::with_capacity(len);

    unsafe {
        input.set_len(len);
        read_args(input.as_mut_ptr());
    }

    // Check for minimum length for selector
    if input.len() < 4 {
        return 1;
    }

    // Extract function selector
    let selector = &input[0..4];
    let payload = &input[4..];

    // Route to appropriate handler based on selector
    let result = match selector {
        sel if sel == SET_COUNT_SELECTOR => {
            handle_set_count(payload);
            Vec::new() // No return value for setCount
        }
        sel if sel == GET_COUNT_SELECTOR => handle_get_count(),
        _ => {
            return 1; // Unknown selector
        }
    };

    unsafe {
        write_result(result.as_ptr(), result.len());
    }

    0
}
