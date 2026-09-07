extern crate alloc;
use alloc::vec::Vec;

use super::vm_context::vm_ctx;
use crate::types::Address;

pub fn set_test_args(args: Vec<u8>) {
    vm_ctx().test_args = args;
}

pub fn get_test_result() -> Vec<u8> {
    vm_ctx().test_result.clone()
}

pub fn get_storage_value(key: &[u8; 32]) -> Option<[u8; 32]> {
    vm_ctx().storage.get(key).cloned()
}

pub fn set_msg_value(value: [u8; 32]) {
    vm_ctx().msg_value = value;
}

pub fn get_msg_value() -> [u8; 32] {
    vm_ctx().msg_value
}

pub fn set_msg_sender(sender: Address) {
    vm_ctx().msg_sender = sender;
}

pub fn set_return_data(data: Vec<Vec<u8>>) {
    let vm_ctx = vm_ctx();
    vm_ctx.return_data = data;
    vm_ctx.return_data_index = 0;
}

pub fn set_msg_reentrant(value: bool) {
    vm_ctx().msg_reentrant = value;
}
