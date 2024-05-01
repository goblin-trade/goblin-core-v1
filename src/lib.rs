#![feature(error_in_core)]

#![cfg_attr(not(test), no_main)]
#![cfg_attr(not(test), no_std)]
extern crate alloc;

pub mod quantities;
pub mod state;
pub mod error;

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use alloc::vec::Vec;
use alloc::vec;

use stylus_sdk::{
    alloy_primitives::{B256, U256},
    console,
    stylus_proc::entrypoint, ArbResult,
};

use crate::{error::{FairyError, InvalidInstructionData}, state::slot_storage::SlotActions};
use state::slot_storage::SlotStorage;

#[entrypoint]
fn main(input: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {

    // let custom_err = FairyError::InvalidInstructionData(InvalidInstructionData {});
    // let custom_err_raw: Vec<u8> = Err(custom_err.into()).;
    return Err(FairyError::InvalidInstructionData(InvalidInstructionData {}).into());

    // let instruction_data = input.as_slice();
    // let (tag, data) = input
    //     .split_first()
    //     // .ok_or(custom_err)?;
    //     .ok_or(vec![0u8])?; // this must be Vec<u8>
    //     // .unwrap();

    // console!("input {:?}", input);


    // let mut slot_storage = SlotStorage::new();

    // let key = [0u8; 32];
    // let data = B256::from(U256::from(1));

    // let data_bytes = data.0;
    // console!("Storing {:?}", data_bytes);

    // // store data
    // slot_storage.sstore(&key, &data.0);

    // // read data
    // let read_data = slot_storage.sload(&key);

    // console!("Read {:?}", read_data);

    // Ok(read_data.to_vec())
}
