extern crate alloc;
use crate::hostio::hostio_unsafe::tests::*;

#[no_mangle]
pub unsafe extern "C" fn read_args(dest: *mut u8) {
    TEST_ARGS.with(|test_args| {
        let args = test_args.borrow();
        let slice = core::slice::from_raw_parts_mut(dest, args.len());
        slice.copy_from_slice(&args);
    });
}

#[no_mangle]
pub unsafe extern "C" fn write_result(data: *const u8, len: usize) {
    TEST_RESULT.with(|test_result| {
        let slice = core::slice::from_raw_parts(data, len);
        *test_result.borrow_mut() = slice.to_vec();
    });
}

#[no_mangle]
pub unsafe extern "C" fn pay_for_memory_grow(_pages: u16) {
    // No-op in test environment
}

#[no_mangle]
pub unsafe extern "C" fn msg_value(value: *mut u8) {
    MSG_VALUE.with(|msg_value| {
        let slice = core::slice::from_raw_parts_mut(value, 32);
        slice.copy_from_slice(&*msg_value.borrow());
    });
}

#[no_mangle]
pub unsafe extern "C" fn msg_sender(sender: *mut u8) {
    MSG_SENDER.with(|addr| {
        let slice = core::slice::from_raw_parts_mut(sender, 20);
        slice.copy_from_slice(&*addr.borrow());
    });
}

#[no_mangle]
pub unsafe extern "C" fn msg_reentrant() -> bool {
    MSG_REENTRANT.with(|flag| *flag.borrow())
}
