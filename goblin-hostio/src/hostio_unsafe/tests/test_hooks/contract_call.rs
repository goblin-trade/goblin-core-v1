extern crate alloc;
use crate::hostio_unsafe::tests::*;

#[no_mangle]
pub unsafe extern "C" fn call_contract(
    _contract: *const u8,
    _calldata: *const u8,
    _calldata_len: usize,
    _value: *const u8,
    _gas: u64,
    return_data_len: *mut usize,
) -> u8 {
    let vm_ctx = vm_ctx();

    if vm_ctx.return_data_index >= vm_ctx.return_data.len() {
        *return_data_len = 0;
    } else {
        let data = &vm_ctx.return_data[vm_ctx.return_data_index];
        *return_data_len = data.len();
        vm_ctx.return_data_index += 1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn static_call_contract(
    _contract: *const u8,
    _calldata: *const u8,
    _calldata_len: usize,
    _gas: u64,
    return_data_len: *mut usize,
) -> u8 {
    let vm_ctx = vm_ctx();

    if vm_ctx.return_data_index >= vm_ctx.return_data.len() {
        *return_data_len = 0;
    } else {
        let data = &vm_ctx.return_data[vm_ctx.return_data_index];
        *return_data_len = data.len();
        vm_ctx.return_data_index += 1;
    }

    0
}

// Returns the queued return data. It should only be called after calling call_contract()
// or static_call_contract(); otherwise it returns 0.
#[no_mangle]
pub unsafe extern "C" fn read_return_data(dest: *mut u8, offset: usize, size: usize) -> usize {
    let vm_ctx = vm_ctx();

    // index == 0 means no call has occurred yet
    if vm_ctx.return_data_index == 0 || vm_ctx.return_data_index > vm_ctx.return_data.len() {
        return 0;
    }

    let data = &vm_ctx.return_data[vm_ctx.return_data_index - 1];

    if offset >= data.len() {
        return 0;
    }

    let end = (offset + size).min(data.len());
    let slice = &data[offset..end];

    let dest_slice = core::slice::from_raw_parts_mut(dest, slice.len());
    dest_slice.copy_from_slice(slice);

    slice.len()
}
