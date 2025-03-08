// VM hooks
#[cfg(not(test))]
#[link(wasm_import_module = "vm_hooks")]
extern "C" {
    pub fn read_args(dest: *mut u8);
    pub fn write_result(data: *const u8, len: usize);
    pub fn pay_for_memory_grow(pages: u16);
    pub fn storage_load_bytes32(key: *const u8, dest: *mut u8);
    pub fn storage_cache_bytes32(key: *const u8, value: *const u8);
    pub fn storage_flush_cache(clear: bool);
    pub fn native_keccak256(bytes: *const u8, len: usize, output: *mut u8);
    pub fn msg_value(value: *mut u8);
    pub fn msg_sender(sender: *mut u8);
    pub fn call_contract(
        contract: *const u8,
        calldata: *const u8,
        calldata_len: usize,
        value: *const u8,
        gas: u64,
        return_data_len: *mut usize,
    ) -> u8;
    pub fn read_return_data(dest: *mut u8, offset: usize, size: usize) -> usize;
}

// #[cfg(not(test))]
// #[link(wasm_import_module = "console")]
// extern "C" {
//     pub fn log_i64(value: i64);

//     /// Prints a UTF-8 encoded string to the console. Only available in debug mode.
//     pub fn log_txt(text: *const u8, len: usize);
// }

#[cfg(test)]
mod test_hooks {
    extern crate alloc;
    use alloc::vec::Vec;
    use core::cell::RefCell;
    use std::collections::HashMap;
    use tiny_keccak::{Hasher, Keccak};

    thread_local! {
        // Store the input args that will be read by read_args
        static TEST_ARGS: RefCell<Vec<u8>> = RefCell::new(Vec::new());

        // Store the result written by write_result
        static TEST_RESULT: RefCell<Vec<u8>> = RefCell::new(Vec::new());

        // Store key-value pairs for storage simulation
        static STORAGE: RefCell<HashMap<[u8; 32], [u8; 32]>> = RefCell::new(HashMap::new());

        // Store the message value
        static MSG_VALUE: RefCell<[u8; 32]> = RefCell::new([0u8; 32]);

        // Add storage for sender address
        static MSG_SENDER: RefCell<[u8; 32]> = RefCell::new([0u8; 32]);

        // Simulate contract call return data
        static RETURN_DATA: RefCell<Vec<u8>> = RefCell::new(Vec::new());
    }

    pub fn set_test_args(args: Vec<u8>) {
        TEST_ARGS.with(|test_args| {
            *test_args.borrow_mut() = args;
        });
    }

    pub fn get_test_result() -> Vec<u8> {
        TEST_RESULT.with(|test_result| test_result.borrow().clone())
    }

    pub fn get_storage_value(key: &[u8; 32]) -> Option<[u8; 32]> {
        STORAGE.with(|storage| storage.borrow().get(key).cloned())
    }

    pub fn set_msg_value(value: [u8; 32]) {
        MSG_VALUE.with(|msg_value| {
            *msg_value.borrow_mut() = value;
        });
    }

    pub fn get_msg_value() -> [u8; 32] {
        MSG_VALUE.with(|msg_value| *msg_value.borrow())
    }

    pub fn clear_state() {
        TEST_ARGS.with(|args| args.borrow_mut().clear());
        TEST_RESULT.with(|result| result.borrow_mut().clear());
        STORAGE.with(|storage| storage.borrow_mut().clear());
        MSG_VALUE.with(|msg_value| *msg_value.borrow_mut() = [0u8; 32]);
        MSG_SENDER.with(|sender| *sender.borrow_mut() = [0u8; 32]);
    }

    // Function to set the test sender address
    pub fn set_msg_sender(sender: [u8; 32]) {
        MSG_SENDER.with(|addr| {
            *addr.borrow_mut() = sender;
        });
    }

    pub fn set_return_data(data: Vec<u8>) {
        RETURN_DATA.with(|return_data| {
            *return_data.borrow_mut() = data;
        });
    }

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
    pub unsafe extern "C" fn storage_load_bytes32(key: *const u8, dest: *mut u8) {
        let key_slice = core::slice::from_raw_parts(key, 32);
        let mut key_array = [0u8; 32];
        key_array.copy_from_slice(key_slice);

        // Create a mutable slice for the destination
        let dest_slice = core::slice::from_raw_parts_mut(dest, 32);

        if let Some(value) = get_storage_value(&key_array) {
            dest_slice.copy_from_slice(&value);
        } else {
            // Zero-fill the destination if no value is found
            dest_slice.fill(0);
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn storage_cache_bytes32(key: *const u8, value: *const u8) {
        STORAGE.with(|storage| {
            let key_slice = core::slice::from_raw_parts(key, 32);
            let mut key_array = [0u8; 32];
            key_array.copy_from_slice(key_slice);

            let value_slice = core::slice::from_raw_parts(value, 32);
            let mut value_array = [0u8; 32];
            value_array.copy_from_slice(value_slice);

            storage.borrow_mut().insert(key_array, value_array);
        });
    }

    #[no_mangle]
    pub unsafe extern "C" fn storage_flush_cache(_clear: bool) {
        // In test environment, we don't need to distinguish between cached and flushed state
    }

    #[no_mangle]
    pub unsafe extern "C" fn log_i64(value: i64) {
        println!("i64({})", value);
    }

    #[no_mangle]
    pub unsafe extern "C" fn log_txt(text: *const u8, len: usize) {
        let slice = core::slice::from_raw_parts(text, len);
        if let Ok(text) = core::str::from_utf8(slice) {
            println!("Stylus says: {}", text);
        }
    }

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
            let slice = core::slice::from_raw_parts_mut(sender, 32);
            slice.copy_from_slice(&*addr.borrow());
        });
    }

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
            let data = return_data.borrow();
            *return_data_len = data.len();
        });
        0 // Indicate success
    }

    #[no_mangle]
    pub unsafe extern "C" fn read_return_data(dest: *mut u8, offset: usize, size: usize) -> usize {
        RETURN_DATA.with(|return_data| {
            let data = return_data.borrow();
            if offset >= data.len() {
                return 0; // Out of bounds
            }
            let end = (offset + size).min(data.len());
            let slice = &data[offset..end];
            let dest_slice = core::slice::from_raw_parts_mut(dest, slice.len());
            dest_slice.copy_from_slice(slice);
            slice.len()
        })
    }
}

#[cfg(test)]
pub use test_hooks::*;

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn test_msg_value() {
        let mut value = [0u8; 32];
        unsafe {
            msg_value(value.as_mut_ptr());
        }
        assert_eq!(value, [0u8; 32]);

        set_msg_value([1u8; 32]);
        unsafe {
            msg_value(value.as_mut_ptr());
        }
        assert_eq!(value, [1u8; 32]);
    }

    #[test]
    fn test_keccak() {
        // Input data
        let input = b"hello world";

        // Expected Keccak-256 hash of "hello world"
        let expected_hash =
            hex!("47173285a8d7341e5e972fc677286384f802f8ef42a5ec5f03bbfa254cb01fad");

        // Output buffer
        let mut output = [0u8; 32];

        // Call the native_keccak256 function
        unsafe {
            native_keccak256(input.as_ptr(), input.len(), output.as_mut_ptr());
        }

        // Verify the output matches the expected hash
        assert_eq!(output, expected_hash);
    }

    #[test]
    fn test_call_contract() {
        set_return_data(vec![1]); // Simulate successful return (true)

        let mut return_data_len = 0;
        let call_result = unsafe {
            call_contract(
                core::ptr::null(),
                core::ptr::null(),
                0,
                core::ptr::null(),
                0,
                &mut return_data_len,
            )
        };

        assert_eq!(call_result, 0);
        assert_eq!(return_data_len, 1);
    }

    #[test]
    fn test_read_return_data() {
        set_return_data(vec![0x12, 0x34, 0x56]);

        let mut buffer = [0u8; 2];
        let bytes_read = unsafe { read_return_data(buffer.as_mut_ptr(), 1, 2) };

        assert_eq!(bytes_read, 2);
        assert_eq!(buffer, [0x34, 0x56]);
    }
}
