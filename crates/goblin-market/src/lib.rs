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
    reduce_order,
};
use quantities::{BaseLots, Ticks, WrapperU64};
use state::{OrderId, RestingOrderIndex, Side, SlotActions, SlotStorage, TraderState};
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

    pub fn reduce_order(
        &mut self,
        is_bid: bool,
        price_in_ticks: u64,
        resting_order_index: u8,
        size: u64,
        recipient: Address,
    ) -> GoblinResult<()> {
        reduce_order::process_reduce_order(
            self,
            Side::init(is_bid),
            &OrderId {
                price_in_ticks: Ticks::new(price_in_ticks),
                resting_order_index: RestingOrderIndex::new(resting_order_index),
            },
            BaseLots::new(size),
            Some(recipient),
        )
    }

    pub fn reduce_order_with_free_funds(
        &mut self,
        is_bid: bool,
        price_in_ticks: u64,
        resting_order_index: u8,
        size: u64,
    ) -> GoblinResult<()> {
        reduce_order::process_reduce_order(
            self,
            Side::init(is_bid),
            &OrderId {
                price_in_ticks: Ticks::new(price_in_ticks),
                resting_order_index: RestingOrderIndex::new(resting_order_index),
            },
            BaseLots::new(size),
            None,
        )
    }

    pub fn cancel_multiple_orders_by_id_with_free_funds(
        &mut self,
        order_ids: Vec<B256>,
    ) -> GoblinResult<()> {
        Ok(())
    }

    // TODO how to return struct? Facing AbiType trait error
    pub fn trader_state(&self, trader: Address) -> B256 {
        let slot_storage = SlotStorage::new();
        let trader_state = TraderState::read_from_slot(&slot_storage, trader);

        B256::from_slice(&trader_state.encode())
    }
}
