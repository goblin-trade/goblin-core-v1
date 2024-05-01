use stylus_sdk::{
    alloy_primitives::{Address, U256},
    // alloy_sol_types::sol,
    evm, msg,
    prelude::*,
};
use alloy_sol_types::sol;

sol! {
    error InvalidInstructionData();
}

#[derive(SolidityError)]
pub enum FairyError {
    InvalidInstructionData(InvalidInstructionData)
}
