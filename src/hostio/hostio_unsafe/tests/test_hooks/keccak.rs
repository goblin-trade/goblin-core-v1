extern crate alloc;
use tiny_keccak::{Hasher, Keccak};

#[no_mangle]
pub unsafe extern "C" fn native_keccak256(bytes: *const u8, len: usize, output: *mut u8) {
    let input_slice = core::slice::from_raw_parts(bytes, len);
    let mut hasher = Keccak::v256();
    hasher.update(input_slice);
    let mut result = [0u8; 32];
    hasher.finalize(&mut result);
    let output_slice = core::slice::from_raw_parts_mut(output, 32);
    output_slice.copy_from_slice(&result);
}
