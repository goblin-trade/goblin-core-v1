#![cfg_attr(not(test), no_main)]
#![cfg_attr(not(test), no_std)]
extern crate alloc;

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use alloc::vec::Vec;

use stylus_sdk::{alloy_primitives::{B256, U256}, console, hostio, stylus_proc::entrypoint};

#[entrypoint]
fn user_main(input: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
    console!("In function");

    let key = B256::ZERO;
    let data = B256::from(U256::from(1));

    // store data
    unsafe {
        hostio::storage_store_bytes32(key.as_ptr(), data.as_ptr())
    };

    // read value
    let mut read_data = B256::ZERO;
    unsafe {
        hostio::storage_load_bytes32(key.as_ptr(), read_data.as_mut_ptr())
    };

    // Decode as big endian. Ethereum stores data in this format
    let read_data_num = U256::from_be_bytes(read_data.0);

    // Big endian- smallest bits are stored towards the end
    let sliced_number = i64::from_be_bytes(read_data.0[24..].try_into().unwrap());

    console!("Read bytes {}, number {}, sliced number {}", read_data, read_data_num, sliced_number);

    Ok(read_data.to_vec())

    // Ok(input)
}
