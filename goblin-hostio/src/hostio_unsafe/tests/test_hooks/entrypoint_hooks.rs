extern crate alloc;
use crate::hostio_unsafe::tests::*;

#[no_mangle]
pub unsafe extern "C" fn read_args(dest: *mut u8) {
    let args = &vm_ctx().test_args;
    let slice = core::slice::from_raw_parts_mut(dest, args.len());
    slice.copy_from_slice(args);
}

#[no_mangle]
pub unsafe extern "C" fn write_result(data: *const u8, len: usize) {
    let slice = core::slice::from_raw_parts(data, len);
    vm_ctx().test_result = slice.to_vec();
}

#[no_mangle]
pub unsafe extern "C" fn pay_for_memory_grow(_pages: u16) {
    // No-op in test environment
}

#[no_mangle]
pub unsafe extern "C" fn msg_value(value: *mut u8) {
    let slice = core::slice::from_raw_parts_mut(value, 32);
    slice.copy_from_slice(&vm_ctx().msg_value);
}

#[no_mangle]
pub unsafe extern "C" fn msg_sender(sender: *mut u8) {
    let slice = core::slice::from_raw_parts_mut(sender, 20);
    slice.copy_from_slice(&vm_ctx().msg_sender);
}

#[no_mangle]
pub unsafe extern "C" fn msg_reentrant() -> bool {
    vm_ctx().msg_reentrant
}
