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
}

#[cfg(not(test))]
#[link(wasm_import_module = "console")]
extern "C" {
    pub fn log_i64(value: i64);

    /// Prints a UTF-8 encoded string to the console. Only available in debug mode.
    pub fn log_txt(text: *const u8, len: usize);
}

#[cfg(test)]
mod test_hooks {
    use alloc::vec::Vec;
    use core::cell::RefCell;
    use std::collections::HashMap;

    thread_local! {
        // Store the input args that will be read by read_args
        static TEST_ARGS: RefCell<Vec<u8>> = RefCell::new(Vec::new());

        // Store the result written by write_result
        static TEST_RESULT: RefCell<Vec<u8>> = RefCell::new(Vec::new());

        // Store key-value pairs for storage simulation
        static STORAGE: RefCell<HashMap<[u8; 32], [u8; 32]>> = RefCell::new(HashMap::new());

        // Store logs for verification
        static LOGS: RefCell<Vec<String>> = RefCell::new(Vec::new());
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

    pub fn get_logs() -> Vec<String> {
        LOGS.with(|logs| logs.borrow().clone())
    }

    pub fn clear_state() {
        TEST_ARGS.with(|args| args.borrow_mut().clear());
        TEST_RESULT.with(|result| result.borrow_mut().clear());
        STORAGE.with(|storage| storage.borrow_mut().clear());
        LOGS.with(|logs| logs.borrow_mut().clear());
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

        if let Some(value) = get_storage_value(&key_array) {
            let dest_slice = core::slice::from_raw_parts_mut(dest, 32);
            dest_slice.copy_from_slice(&value);
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
        LOGS.with(|logs| {
            logs.borrow_mut().push(value.to_string());
        });
    }

    #[no_mangle]
    pub unsafe extern "C" fn log_txt(text: *const u8, len: usize) {
        let slice = core::slice::from_raw_parts(text, len);
        if let Ok(text) = core::str::from_utf8(slice) {
            LOGS.with(|logs| {
                logs.borrow_mut().push(text.to_string());
            });
        }
    }
}

#[cfg(test)]
pub use test_hooks::*;
