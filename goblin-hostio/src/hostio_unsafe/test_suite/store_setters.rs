extern crate alloc;
use alloc::vec::Vec;

use super::vm_context::{CallType, MockResponse, vm_ctx};

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

pub fn set_msg_sender(sender: [u8; 20]) {
    vm_ctx().msg_sender = sender;
}

/// Register a mock response for a state-changing `call_contract`.
///
/// `calldata` is matched as a *prefix* of the outgoing call's calldata, so
/// passing only the 4-byte selector matches calls with any arguments.
///
/// Multiple mocks can be registered; each outgoing call resolves to the first
/// matching mock, so mocks for different addresses, selectors or call types do
/// not interfere with each other.
pub fn set_mock_call(contract: [u8; 20], calldata: Vec<u8>, return_data: Vec<u8>) {
    set_mock_response(CallType::Call, contract, calldata, return_data);
}

/// Register a mock response for a read-only `static_call_contract`.
///
/// `calldata` is matched as a *prefix* of the outgoing call's calldata, so
/// passing only the 4-byte selector matches calls with any arguments.
pub fn set_mock_static_call(contract: [u8; 20], calldata: Vec<u8>, return_data: Vec<u8>) {
    set_mock_response(CallType::StaticCall, contract, calldata, return_data);
}

/// Register a mock response keyed by call type, contract address and calldata.
///
/// This is the general form of [`set_mock_call`] and [`set_mock_static_call`].
pub fn set_mock_response(
    call_type: CallType,
    contract: [u8; 20],
    calldata: Vec<u8>,
    return_data: Vec<u8>,
) {
    vm_ctx().mock_responses.push(MockResponse {
        call_type,
        contract,
        calldata,
        return_data,
    });
}

/// Remove all registered mocks and any return data recorded by a previous call.
pub fn clear_mock_responses() {
    let vm_ctx = vm_ctx();
    vm_ctx.mock_responses.clear();
    vm_ctx.last_return_data.clear();
}

pub fn set_msg_reentrant(value: bool) {
    vm_ctx().msg_reentrant = value;
}
