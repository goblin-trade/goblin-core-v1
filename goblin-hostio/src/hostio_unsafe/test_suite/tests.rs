use super::*;
use hex_literal::hex;
use std::sync::Mutex;

/// The VM context is a shared `static mut`, so tests that register mocks or
/// otherwise mutate it must not run concurrently.
static TEST_LOCK: Mutex<()> = Mutex::new(());

fn lock() -> std::sync::MutexGuard<'static, ()> {
    TEST_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// Build a 32-byte, big-endian-padded word with `value` in the last byte.
fn word(value: u8) -> Vec<u8> {
    let mut word = vec![0u8; 32];
    word[31] = value;
    word
}

#[test]
fn test_msg_value() {
    let _guard = lock();

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
    let expected_hash = hex!("47173285a8d7341e5e972fc677286384f802f8ef42a5ec5f03bbfa254cb01fad");

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
    let _guard = lock();
    clear_mock_responses();

    let contract = [0x11u8; 20];
    let calldata = [0xaa, 0xbb, 0xcc, 0xdd];
    set_mock_call(contract, calldata.to_vec(), vec![1]);

    let mut return_data_len = 0;
    let call_result = unsafe {
        call_contract(
            contract.as_ptr(),
            calldata.as_ptr(),
            calldata.len(),
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
    let _guard = lock();
    clear_mock_responses();

    let contract = [0x22u8; 20];
    let calldata = [0x01, 0x02, 0x03, 0x04];
    set_mock_call(contract, calldata.to_vec(), vec![0x12, 0x34, 0x56]);

    let mut return_data_len = 0;
    unsafe {
        call_contract(
            contract.as_ptr(),
            calldata.as_ptr(),
            calldata.len(),
            core::ptr::null(),
            0,
            &mut return_data_len,
        )
    };
    assert_eq!(return_data_len, 3);

    let mut buffer = [0u8; 2];
    let bytes_read = unsafe { read_return_data(buffer.as_mut_ptr(), 1, 2) };

    assert_eq!(bytes_read, 2);
    assert_eq!(buffer, [0x34, 0x56]);
}

#[test]
fn test_mocks_are_keyed() {
    let _guard = lock();
    clear_mock_responses();

    let token_a = [0xa1u8; 20];
    let token_b = [0xb2u8; 20];
    let transfer_from = hex!("23b872dd"); // transferFrom(address,address,uint256)
    let decimals = hex!("313ce567"); // decimals()

    // Same selector on different tokens resolves to a different response, and a
    // static call is keyed separately from a regular call.
    set_mock_call(token_a, transfer_from.to_vec(), word(1)); // true
    set_mock_call(token_b, transfer_from.to_vec(), word(0)); // false
    set_mock_static_call(token_a, decimals.to_vec(), word(18));

    // A full `transferFrom` calldata still matches the selector-only mock.
    let mut transfer_calldata = [0u8; 4 + 96];
    transfer_calldata[..4].copy_from_slice(&transfer_from);

    let mut len = 0usize;
    unsafe {
        call_contract(
            token_b.as_ptr(),
            transfer_calldata.as_ptr(),
            transfer_calldata.len(),
            core::ptr::null(),
            0,
            &mut len,
        )
    };
    assert_eq!(len, 32);

    let mut result = [0u8; 1];
    unsafe { read_return_data(result.as_mut_ptr(), 31, 1) };
    assert_eq!(result, [0]);

    unsafe {
        call_contract(
            token_a.as_ptr(),
            transfer_calldata.as_ptr(),
            transfer_calldata.len(),
            core::ptr::null(),
            0,
            &mut len,
        )
    };
    unsafe { read_return_data(result.as_mut_ptr(), 31, 1) };
    assert_eq!(result, [1]);

    unsafe {
        static_call_contract(
            token_a.as_ptr(),
            decimals.as_ptr(),
            decimals.len(),
            0,
            &mut len,
        )
    };
    unsafe { read_return_data(result.as_mut_ptr(), 31, 1) };
    assert_eq!(result, [18]);
}

#[test]
fn test_unmatched_call_returns_empty() {
    let _guard = lock();
    clear_mock_responses();

    let contract = [0x33u8; 20];
    let calldata = [0xde, 0xad, 0xbe, 0xef];

    let mut return_data_len = 0;
    unsafe {
        call_contract(
            contract.as_ptr(),
            calldata.as_ptr(),
            calldata.len(),
            core::ptr::null(),
            0,
            &mut return_data_len,
        )
    };

    assert_eq!(return_data_len, 0);

    let mut buffer = [0xffu8; 1];
    let bytes_read = unsafe { read_return_data(buffer.as_mut_ptr(), 0, 1) };
    assert_eq!(bytes_read, 0);
}
