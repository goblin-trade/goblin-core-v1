extern crate alloc;
use crate::hostio::hostio_unsafe::tests::*;

pub fn set_block_number(value: u64) {
    BLOCK_NUMBER.with(|b| *b.borrow_mut() = value);
}

pub fn set_block_timestamp(value: u64) {
    BLOCK_TIMESTAMP.with(|t| *t.borrow_mut() = value);
}

#[no_mangle]
pub unsafe extern "C" fn block_number() -> u64 {
    BLOCK_NUMBER.with(|b| *b.borrow())
}

#[no_mangle]
pub unsafe extern "C" fn block_timestamp() -> u64 {
    BLOCK_TIMESTAMP.with(|t| *t.borrow())
}
