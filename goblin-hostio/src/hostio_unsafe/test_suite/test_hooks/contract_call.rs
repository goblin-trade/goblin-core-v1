extern crate alloc;
use alloc::vec::Vec;

use crate::hostio_unsafe::test_suite::*;

/// Resolve the mocked return data for an outgoing call, record it as the most
/// recent return data, and write its length to `return_data_len`.
///
/// If no mock matches, the recorded return data is empty and `return_data_len`
/// is set to 0, which lets callers such as `call_and_check` reject the call.
///
/// # Safety
///
/// * `contract` is either null or points to a 20-byte address.
/// * `calldata` is either null or points to `calldata_len` readable bytes.
/// * `return_data_len` points to a writable `usize`.
unsafe fn record_call(
    call_type: CallType,
    contract: *const u8,
    calldata: *const u8,
    calldata_len: usize,
    return_data_len: *mut usize,
) {
    let contract = if contract.is_null() {
        [0u8; 20]
    } else {
        // SAFETY: per the hostio ABI, `contract` points to a 20-byte address.
        unsafe { *(contract as *const [u8; 20]) }
    };

    let calldata = if calldata.is_null() || calldata_len == 0 {
        &[][..]
    } else {
        // SAFETY: `calldata` points to `calldata_len` readable bytes.
        unsafe { core::slice::from_raw_parts(calldata, calldata_len) }
    };

    let vm_ctx = vm_ctx();
    let data = match vm_ctx.find_mock(call_type, &contract, calldata) {
        Some(mock) => mock.return_data.clone(),
        None => Vec::new(),
    };

    // SAFETY: `return_data_len` is a valid out-pointer per the hostio ABI.
    unsafe {
        *return_data_len = data.len();
    }
    vm_ctx.last_return_data = data;
}

/// Emulate a contract call in the test environment, returning the result length.
///
/// The return data is looked up from the mocks registered with `set_mock_call`.
///
/// # Safety
///
/// Developer sets the mock response during initialization.
///
pub unsafe fn call_contract(
    contract: *const u8,
    calldata: *const u8,
    calldata_len: usize,
    _value: *const u8,
    _gas: u64,
    return_data_len: *mut usize,
) -> u8 {
    // SAFETY: pointer invariants are upheld by the hostio ABI.
    unsafe {
        record_call(
            CallType::Call,
            contract,
            calldata,
            calldata_len,
            return_data_len,
        );
    }

    0
}

/// Emulate a static contract call in the test environment, returning the result length.
///
/// The return data is looked up from the mocks registered with
/// `set_mock_static_call`.
///
/// # Safety
///
/// Developer sets the mock response during initialization.
///
pub unsafe fn static_call_contract(
    contract: *const u8,
    calldata: *const u8,
    calldata_len: usize,
    _gas: u64,
    return_data_len: *mut usize,
) -> u8 {
    // SAFETY: pointer invariants are upheld by the hostio ABI.
    unsafe {
        record_call(
            CallType::StaticCall,
            contract,
            calldata,
            calldata_len,
            return_data_len,
        );
    }

    0
}

/// Returns the return data recorded by the most recent `call_contract()` or
/// `static_call_contract()`; otherwise it returns 0.
///
/// # Safety
///
/// Developer sets the mock response during initialization.
///
pub unsafe fn read_return_data(dest: *mut u8, offset: usize, size: usize) -> usize {
    let vm_ctx = vm_ctx();
    let data = &vm_ctx.last_return_data;

    if offset >= data.len() {
        return 0;
    }

    let end = (offset + size).min(data.len());
    let slice = &data[offset..end];

    // SAFETY: `dest` must point to a buffer of at least `size` bytes, which the
    // caller of `read_return_data` guarantees.
    let dest_slice = unsafe { core::slice::from_raw_parts_mut(dest, slice.len()) };
    dest_slice.copy_from_slice(slice);

    slice.len()
}
