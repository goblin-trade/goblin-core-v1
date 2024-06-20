#![cfg_attr(all(not(feature = "export-abi"), not(test)), no_main)]
#![cfg_attr(not(test), no_std)]
extern crate alloc;

pub mod parameters;
pub mod program;
pub mod quantities;
pub mod state;

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use crate::program::GoblinResult;
use alloc::vec::Vec;
use program::{
    processor::{deposit, fees, withdraw},
    reduce_multiple_orders,
};
use state::{SlotActions, SlotStorage, TraderState};
use stylus_sdk::{
    alloy_primitives::{Address, B256},
    prelude::*,
};

sol_storage! {
    #[entrypoint]
    pub struct GoblinMarket {}
}

#[external]
impl GoblinMarket {
    pub fn deposit_funds(
        &mut self,
        trader: Address,
        quote_lots_to_deposit: u64,
        base_lots_to_deposit: u64,
    ) -> GoblinResult<()> {
        deposit::process_deposit_funds(self, trader, quote_lots_to_deposit, base_lots_to_deposit)
    }

    pub fn withdraw_funds(
        &mut self,
        quote_lots_to_withdraw: u64,
        base_lots_to_withdraw: u64,
    ) -> GoblinResult<()> {
        withdraw::process_withdraw_funds(self, quote_lots_to_withdraw, base_lots_to_withdraw)
    }

    pub fn collect_fees(&mut self, recipient: Address) -> GoblinResult<()> {
        fees::process_collect_fees(self, recipient)
    }

    pub fn reduce_multiple_orders(
        &mut self,
        order_packets: Vec<B256>,
        recipient: Address,
    ) -> GoblinResult<()> {
        reduce_multiple_orders::process_reduce_multiple_orders(self, order_packets, Some(recipient))
    }

    pub fn reduce_multiple_orders_with_free_funds(
        &mut self,
        order_packets: Vec<B256>,
    ) -> GoblinResult<()> {
        reduce_multiple_orders::process_reduce_multiple_orders(self, order_packets, None)
    }

    // TODO how to return struct? Facing AbiType trait error
    pub fn trader_state(&self, trader: Address) -> B256 {
        let slot_storage = SlotStorage::new();
        let trader_state = TraderState::read_from_slot(&slot_storage, trader);

        B256::from_slice(&trader_state.encode())
    }
}
