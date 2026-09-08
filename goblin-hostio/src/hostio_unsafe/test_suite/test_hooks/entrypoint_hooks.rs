extern crate alloc;
use crate::hostio_unsafe::test_suite::*;

/// Read args
///
/// # Safety
///
/// Value set by developer during initialization
///
pub unsafe fn read_args(dest: *mut u8) {
    let args = &vm_ctx().test_args;
    let slice = unsafe { core::slice::from_raw_parts_mut(dest, args.len()) };
    slice.copy_from_slice(args);
}

/// Write result of the contract call
///
/// # Safety
///
/// Developer passes correct length of data
///
pub unsafe fn write_result(data: *const u8, len: usize) {
    let slice = unsafe { core::slice::from_raw_parts(data, len) };
    vm_ctx().test_result = slice.to_vec();
}

/// Pay for memory grow. No-op in test environment
pub fn pay_for_memory_grow(_pages: u16) {}

/// Write msg.value into `value` pointer
///
/// # Safety
///
/// Value set by developer during initialization
///
pub unsafe fn msg_value(value: *mut u8) {
    let slice = unsafe { core::slice::from_raw_parts_mut(value, 32) };
    slice.copy_from_slice(&vm_ctx().msg_value);
}

/// Write msg.sender into `value` pointer
///
/// # Safety
///
/// Value set by developer during initialization
///
pub unsafe fn msg_sender(sender: *mut u8) {
    let slice = unsafe { core::slice::from_raw_parts_mut(sender, 20) };
    slice.copy_from_slice(&vm_ctx().msg_sender);
}

/// Whether the call is re-entrant
///
/// # Safety
///
/// Value read from vm context. This value is set during intialization
pub unsafe fn msg_reentrant() -> bool {
    vm_ctx().msg_reentrant
}
