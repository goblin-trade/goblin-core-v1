extern crate alloc;
use crate::hostio::hostio_unsafe::tests::*;

pub fn set_block_number(value: u64) {
    vm_ctx().block_number = value;
}

pub fn set_block_timestamp(value: u64) {
    vm_ctx().block_timestamp = value;
}

#[no_mangle]
pub unsafe extern "C" fn block_number() -> u64 {
    vm_ctx().block_number
}

#[no_mangle]
pub unsafe extern "C" fn block_timestamp() -> u64 {
    vm_ctx().block_timestamp
}
