extern crate alloc;
use super::store::*;
use alloc::vec::Vec;

use crate::types::Address;

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

// Function to set the test sender address
pub fn set_msg_sender(sender: Address) {
    MSG_SENDER.with(|addr| {
        *addr.borrow_mut() = sender;
    });
}

pub fn set_return_data(data: Vec<Vec<u8>>) {
    RETURN_DATA.with(|return_data| {
        *return_data.borrow_mut() = data;
    });
    RETURN_DATA_INDEX.with(|i| *i.borrow_mut() = 0);
}

pub fn set_msg_reentrant(value: bool) {
    MSG_REENTRANT.with(|flag| *flag.borrow_mut() = value);
}
