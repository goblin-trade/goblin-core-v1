extern crate alloc;
use crate::hostio::hostio_unsafe::tests::*;

#[no_mangle]
pub unsafe extern "C" fn call_contract(
    _contract: *const u8,
    _calldata: *const u8,
    _calldata_len: usize,
    _value: *const u8,
    _gas: u64,
    return_data_len: *mut usize,
) -> u8 {
    RETURN_DATA.with(|return_data| {
        RETURN_DATA_INDEX.with(|idx| {
            let mut index = idx.borrow_mut();
            if *index >= return_data.borrow().len() {
                *return_data_len = 0;
            } else {
                let data = &return_data.borrow()[*index];
                *return_data_len = data.len();
                *index += 1;
            }
        });
    });
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
    RETURN_DATA.with(|return_data| {
        RETURN_DATA_INDEX.with(|idx| {
            let mut index = idx.borrow_mut();
            if *index >= return_data.borrow().len() {
                *return_data_len = 0;
            } else {
                let data = &return_data.borrow()[*index];
                *return_data_len = data.len();
                *index += 1;
            }
        });
    });
    0
}

// Returns the queued return data. It should only be called after calling contract_call()
// or static_contract_call() otherwise it will return 0
#[no_mangle]
pub unsafe extern "C" fn read_return_data(dest: *mut u8, offset: usize, size: usize) -> usize {
    RETURN_DATA.with(|return_data| {
        RETURN_DATA_INDEX.with(|idx| {
            let index = *idx.borrow();

            // Returns 0 if index is 0
            if index == 0 || index > return_data.borrow().len() {
                return 0;
            }
            let data = &return_data.borrow()[index - 1];
            if offset >= data.len() {
                return 0;
            }
            let end = (offset + size).min(data.len());
            let slice = &data[offset..end];
            let dest_slice = core::slice::from_raw_parts_mut(dest, slice.len());
            dest_slice.copy_from_slice(slice);
            slice.len()
        })
    })
}
