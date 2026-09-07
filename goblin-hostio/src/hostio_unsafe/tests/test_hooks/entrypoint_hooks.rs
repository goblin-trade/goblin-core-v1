extern crate alloc;
use crate::hostio_unsafe::tests::*;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_args(dest: *mut u8) {
    let args = &vm_ctx().test_args;
    let slice = unsafe { core::slice::from_raw_parts_mut(dest, args.len()) };
    slice.copy_from_slice(args);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_result(data: *const u8, len: usize) {
    let slice = unsafe { core::slice::from_raw_parts(data, len) };
    vm_ctx().test_result = slice.to_vec();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pay_for_memory_grow(_pages: u16) {
    // No-op in test environment
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn msg_value(value: *mut u8) {
    let slice = unsafe { core::slice::from_raw_parts_mut(value, 32) };
    slice.copy_from_slice(&vm_ctx().msg_value);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn msg_sender(sender: *mut u8) {
    let slice = unsafe { core::slice::from_raw_parts_mut(sender, 20) };
    slice.copy_from_slice(&vm_ctx().msg_sender);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn msg_reentrant() -> bool {
    vm_ctx().msg_reentrant
}
