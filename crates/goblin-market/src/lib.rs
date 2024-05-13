#![cfg_attr(not(test), no_main)]
#![cfg_attr(not(test), no_std)]
extern crate alloc;

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use alloc::vec::Vec;
use processor::deposit;
use stylus_sdk::prelude::*;

pub mod processor;
pub mod quantities;
pub mod state;

sol_storage! {
    #[entrypoint]
    pub struct GoblinMarket {}
}

#[external]
impl GoblinMarket {
    pub fn deposit_funds(
        &mut self,
        base_lots_to_deposit: u64,
        quote_lots_to_deposit: u64,
    ) -> Result<(), Vec<u8>> {
        deposit::process_deposit_funds(base_lots_to_deposit, quote_lots_to_deposit);
        Ok(())
    }
}
