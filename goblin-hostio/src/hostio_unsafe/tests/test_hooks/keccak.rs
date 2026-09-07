extern crate alloc;
use keccak_const::Keccak256;

#[no_mangle]
pub unsafe extern "C" fn native_keccak256(bytes: *const u8, len: usize, output: *mut u8) {
    let input_slice = core::slice::from_raw_parts(bytes, len);
    let result = Keccak256::new().update(input_slice).finalize();
    let output_slice = core::slice::from_raw_parts_mut(output, 32);
    output_slice.copy_from_slice(&result);
}
