#![allow(static_mut_refs)]

extern crate alloc;
use alloc::vec::Vec;
use std::collections::BTreeMap;

pub struct VMContext {
    pub test_args: Vec<u8>,
    pub test_result: Vec<u8>,
    pub storage: BTreeMap<[u8; 32], [u8; 32]>,
    pub msg_value: [u8; 32],
    pub msg_sender: [u8; 20],
    pub block_number: u64,
    pub block_timestamp: u64,
    pub return_data: Vec<Vec<u8>>,
    pub return_data_index: usize,
    pub msg_reentrant: bool,
}

impl VMContext {
    pub const fn new() -> Self {
        Self {
            test_args: Vec::new(),
            test_result: Vec::new(),
            storage: BTreeMap::new(),
            msg_value: [0u8; 32],
            msg_sender: [0u8; 20],
            block_number: 0,
            block_timestamp: 0,
            return_data: Vec::new(),
            return_data_index: 0,
            msg_reentrant: false,
        }
    }
}

static mut VM_CTX: VMContext = VMContext::new();

pub fn vm_ctx() -> &'static mut VMContext {
    unsafe { &mut VM_CTX }
}
