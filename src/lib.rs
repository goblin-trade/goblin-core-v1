#![cfg_attr(not(test), no_main)]
#![cfg_attr(not(test), no_std)]
extern crate alloc;

pub mod quantities;
pub mod state;
pub mod error;
pub mod instruction;

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use alloc::vec::Vec;

use stylus_sdk::{
    console, stylus_proc::entrypoint
};

use crate::{error::{FairyError, InvalidInstructionData}, state::slot_storage::SlotActions};

#[entrypoint]
fn main(instruction_data: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
    let (tag, data) = instruction_data.split_first()
        .ok_or(FairyError::InvalidInstructionData(InvalidInstructionData {}))?;

    console!("input {:?}", instruction_data);
    console!("tag {}", *tag);

    Ok(instruction_data.to_vec())
}
