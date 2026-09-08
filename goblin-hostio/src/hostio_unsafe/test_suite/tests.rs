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
    set_return_data(vec![
        vec![1], // Simulate successful return (true)
    ]);

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
    set_return_data(vec![vec![0x12, 0x34, 0x56]]);

    let mut return_data_len = 0;
    unsafe {
        call_contract(
            core::ptr::null(),
            core::ptr::null(),
            0,
            core::ptr::null(),
            0,
            &mut return_data_len,
        )
    };
    assert_eq!(return_data_len, 3);

    let mut buffer = [0u8; 2];
    let bytes_read = unsafe { read_return_data(buffer.as_mut_ptr(), 1, 2) };

    assert_eq!(bytes_read, 2);
    // assert_eq!(buffer, [0x34, 0x56]);
}
