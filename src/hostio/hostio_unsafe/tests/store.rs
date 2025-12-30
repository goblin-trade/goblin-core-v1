extern crate alloc;
use alloc::vec::Vec;
use core::cell::RefCell;
use std::collections::HashMap;

use crate::types::Address;

thread_local! {
    // Store the input args that will be read by read_args
    pub static TEST_ARGS: RefCell<Vec<u8>> = RefCell::new(Vec::new());

    // Store the result written by write_result
    pub static TEST_RESULT: RefCell<Vec<u8>> = RefCell::new(Vec::new());

    // Store key-value pairs for storage simulation
    pub static STORAGE: RefCell<HashMap<[u8; 32], [u8; 32]>> = RefCell::new(HashMap::new());

    // Store the message value
    pub static MSG_VALUE: RefCell<[u8; 32]> = RefCell::new([0u8; 32]);

    // Add storage for sender address
    pub static MSG_SENDER: RefCell<Address> = RefCell::new([0u8; 20]);

    pub static BLOCK_NUMBER: RefCell<u64> = RefCell::new(0);

    pub static BLOCK_TIMESTAMP: RefCell<u64> = RefCell::new(0);

    // Simulate contract call return data
    // Holds an array of values to be returned
    pub static RETURN_DATA: RefCell<Vec<Vec<u8>>> = RefCell::new(Vec::new());

    // Starts with 0. Is incremented whenever call_contract() is called.
    // Calling read_return_data() without calling call_contract() first gives 0
    pub static RETURN_DATA_INDEX: RefCell<usize> = RefCell::new(0);

    pub static MSG_REENTRANT: RefCell<bool> = RefCell::new(false);
}

pub fn clear_store() {
    TEST_ARGS.with(|args| args.borrow_mut().clear());
    TEST_RESULT.with(|result| result.borrow_mut().clear());
    STORAGE.with(|storage| storage.borrow_mut().clear());
    MSG_VALUE.with(|msg_value| *msg_value.borrow_mut() = [0u8; 32]);
    MSG_SENDER.with(|sender| *sender.borrow_mut() = [0u8; 20]);
    BLOCK_NUMBER.with(|b| *b.borrow_mut() = 0);
    BLOCK_TIMESTAMP.with(|t| *t.borrow_mut() = 0);
    RETURN_DATA.with(|data| data.borrow_mut().clear());
    RETURN_DATA_INDEX.with(|i| *i.borrow_mut() = 0);
}
