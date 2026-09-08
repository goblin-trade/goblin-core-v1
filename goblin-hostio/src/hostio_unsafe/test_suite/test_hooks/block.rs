extern crate alloc;
use crate::hostio_unsafe::test_suite::*;

pub fn set_block_number(value: u64) {
    vm_ctx().block_number = value;
}

pub fn set_block_timestamp(value: u64) {
    vm_ctx().block_timestamp = value;
}

/// Get block number
///
/// # Safety
///
/// Read from vm context
pub unsafe fn block_number() -> u64 {
    vm_ctx().block_number
}

/// Get block timestamp
///
/// # Safety
///
/// Read from vm context
pub unsafe fn block_timestamp() -> u64 {
    vm_ctx().block_timestamp
}
