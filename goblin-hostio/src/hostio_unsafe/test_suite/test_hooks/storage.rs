extern crate alloc;
use crate::hostio_unsafe::test_suite::*;

/// Write the value stored at `key` into `dest`
///
/// # Safety
///
/// * `key` and `dest` are pointers to 32 byte byte arrays
/// * Test suite reads from an in-memory K-V store
pub unsafe fn storage_load_bytes32(key: *const u8, dest: *mut u8) {
    let key_slice = unsafe { core::slice::from_raw_parts(key, 32) };
    let mut key_array = [0u8; 32];
    key_array.copy_from_slice(key_slice);

    // Create a mutable slice for the destination
    let dest_slice = unsafe { core::slice::from_raw_parts_mut(dest, 32) };

    if let Some(value) = get_storage_value(&key_array) {
        dest_slice.copy_from_slice(&value);
    } else {
        // Zero-fill the destination if no value is found
        dest_slice.fill(0);
    }
}

/// Write the key value pair to storage
///
/// # Safety
///
/// * `key` and `value` are pointers to 32 byte byte arrays
/// * Test suite writes to an in-memory K-V store
///
pub unsafe fn storage_cache_bytes32(key: *const u8, value: *const u8) {
    let key_slice = unsafe { core::slice::from_raw_parts(key, 32) };
    let mut key_array = [0u8; 32];
    key_array.copy_from_slice(key_slice);

    let value_slice = unsafe { core::slice::from_raw_parts(value, 32) };
    let mut value_array = [0u8; 32];
    value_array.copy_from_slice(value_slice);

    vm_ctx().storage.insert(key_array, value_array);
}

/// Flush cache. No-op in test environment
///
/// # Safety
///
/// No-op in test environment
pub unsafe fn storage_flush_cache(_clear: bool) {}
