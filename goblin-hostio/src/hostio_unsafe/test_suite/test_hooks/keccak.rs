extern crate alloc;
use keccak_const::Keccak256;

/// Calculate keccak256 hash and write result to `output`
///
/// # Safety
///
/// Correct length is passed
pub unsafe fn native_keccak256(bytes: *const u8, len: usize, output: *mut u8) {
    let input_slice = unsafe { core::slice::from_raw_parts(bytes, len) };
    let result = Keccak256::new().update(input_slice).finalize();
    let output_slice = unsafe { core::slice::from_raw_parts_mut(output, 32) };
    output_slice.copy_from_slice(&result);
}
