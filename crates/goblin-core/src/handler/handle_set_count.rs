use crate::{hostio::*, COUNTER_KEY};

pub fn handle_set_count(input: &[u8]) {
    if input.len() != 32 {
        return;
    }

    // Read the uint256 parameter (we'll only use the last 8 bytes for u64)
    let value = u64::from_be_bytes(input[24..32].try_into().unwrap());
    write_counter(value);
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
