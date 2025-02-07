use crate::{hostio::*, COUNTER_KEY};
use alloc::vec::Vec;

pub fn handle_get_count() -> Vec<u8> {
    let value = read_counter();
    let mut result = Vec::with_capacity(32);
    result.extend_from_slice(&[0u8; 24]); // Pad with zeros
    result.extend_from_slice(&value.to_be_bytes()); // Append the u64 value
    result
}

// Helper function to read counter value from storage
fn read_counter() -> u64 {
    let mut value = [0u8; 32];
    unsafe {
        storage_load_bytes32(COUNTER_KEY.as_ptr(), value.as_mut_ptr());
    }
    u64::from_be_bytes(value[24..32].try_into().unwrap())
}
