extern crate alloc;
use crate::hostio::hostio_unsafe::tests::*;

#[no_mangle]
pub unsafe extern "C" fn storage_load_bytes32(key: *const u8, dest: *mut u8) {
    let key_slice = core::slice::from_raw_parts(key, 32);
    let mut key_array = [0u8; 32];
    key_array.copy_from_slice(key_slice);

    // Create a mutable slice for the destination
    let dest_slice = core::slice::from_raw_parts_mut(dest, 32);

    if let Some(value) = get_storage_value(&key_array) {
        dest_slice.copy_from_slice(&value);
    } else {
        // Zero-fill the destination if no value is found
        dest_slice.fill(0);
    }
}

#[no_mangle]
pub unsafe extern "C" fn storage_cache_bytes32(key: *const u8, value: *const u8) {
    STORAGE.with(|storage| {
        let key_slice = core::slice::from_raw_parts(key, 32);
        let mut key_array = [0u8; 32];
        key_array.copy_from_slice(key_slice);

        let value_slice = core::slice::from_raw_parts(value, 32);
        let mut value_array = [0u8; 32];
        value_array.copy_from_slice(value_slice);

        storage.borrow_mut().insert(key_array, value_array);
    });
}

#[no_mangle]
pub unsafe extern "C" fn storage_flush_cache(_clear: bool) {
    // In test environment, we don't need to distinguish between cached and flushed state
}
