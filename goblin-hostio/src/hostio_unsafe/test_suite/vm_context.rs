#![allow(static_mut_refs)]

extern crate alloc;
use alloc::vec::Vec;
use std::collections::BTreeMap;

/// Identifies which contract-call hostio a [`MockResponse`] targets.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CallType {
    /// A state-changing `call_contract`.
    Call,
    /// A read-only `static_call_contract`.
    StaticCall,
}

/// A mocked response for a contract call made during a test.
///
/// A mock matches an outgoing call when the [`CallType`] and `contract` address
/// are equal and `calldata` is a prefix of the call's calldata. Registering only
/// the 4-byte selector (e.g. `decimals()`) therefore matches the call regardless
/// of its arguments, while registering the full calldata pins exact arguments.
#[derive(Clone)]
pub struct MockResponse {
    pub call_type: CallType,
    pub contract: [u8; 20],
    pub calldata: Vec<u8>,
    pub return_data: Vec<u8>,
}

#[derive(Default)]
pub struct VMContext {
    pub test_args: Vec<u8>,
    pub test_result: Vec<u8>,
    pub storage: BTreeMap<[u8; 32], [u8; 32]>,
    pub msg_value: [u8; 32],
    pub msg_sender: [u8; 20],
    pub block_number: u64,
    pub block_timestamp: u64,
    pub mock_responses: Vec<MockResponse>,
    pub last_return_data: Vec<u8>,
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
            mock_responses: Vec::new(),
            last_return_data: Vec::new(),
            msg_reentrant: false,
        }
    }

    /// Find the first registered mock matching the given call.
    ///
    /// Matching requires an equal `call_type`, an equal `contract` address, and
    /// a registered `calldata` that is a prefix of the call's `calldata`.
    pub fn find_mock(
        &self,
        call_type: CallType,
        contract: &[u8; 20],
        calldata: &[u8],
    ) -> Option<&MockResponse> {
        self.mock_responses.iter().find(|mock| {
            mock.call_type == call_type
                && mock.contract == *contract
                && calldata.starts_with(&mock.calldata)
        })
    }
}

static mut VM_CTX: VMContext = VMContext::new();

pub fn vm_ctx() -> &'static mut VMContext {
    unsafe { &mut VM_CTX }
}
