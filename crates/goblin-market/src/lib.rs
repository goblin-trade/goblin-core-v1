#![cfg_attr(all(not(feature = "export-abi"), not(test)), no_main)]
#![cfg_attr(not(test), no_std)]
extern crate alloc;

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use processor::deposit;
use stylus_sdk::{alloy_primitives::Address, prelude::*};

pub mod error;
pub mod parameters;
pub mod processor;
pub mod quantities;
pub mod state;

use crate::error::GoblinResult;

sol_storage! {
    #[entrypoint]
    pub struct GoblinMarket {}
}

#[external]
impl GoblinMarket {
    pub fn deposit_funds(
        &mut self,
        trader: Address,
        base_lots_to_deposit: u64,
        quote_lots_to_deposit: u64,
    ) -> GoblinResult<()> {
        deposit::process_deposit_funds(trader, base_lots_to_deposit, quote_lots_to_deposit)?;
        Ok(())
    }
}
