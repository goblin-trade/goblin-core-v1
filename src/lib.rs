#![cfg_attr(not(test), no_main)]
#![cfg_attr(not(test), no_std)]
extern crate alloc;

pub mod quantities;
pub mod state;

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use alloc::vec::Vec;

use stylus_sdk::{alloy_primitives::{B256, U256}, console, stylus_proc::entrypoint};

use state::slot_storage::SlotStorage;
use crate::state::slot_storage::SlotActions;

#[entrypoint]
fn user_main(input: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
    let mut slot_storage = SlotStorage::new();

    let key = [0u8; 32];
    let data = B256::from(U256::from(1));

    let data_bytes = data.0;
    console!("Storing {:?}", data_bytes);

    // store data
    slot_storage.sstore(&key, &data.0);

    // read data
    let read_data = slot_storage.sload(&key);

    console!("Read {:?}", read_data);

    Ok(read_data.to_vec())
}
