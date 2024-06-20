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
    new_order,
    processor::{deposit, fees, withdraw},
    reduce_multiple_orders, FailedMultipleLimitOrderBehavior,
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

    /// Reduce multiple orders and withdraw the funds to recipient address
    pub fn reduce_multiple_orders(
        &mut self,
        order_packets: Vec<B256>,
        recipient: Address,
    ) -> GoblinResult<()> {
        reduce_multiple_orders::process_reduce_multiple_orders(self, order_packets, Some(recipient))
    }

    /// Reduce multiple orders. Retain the funds with the exchange
    pub fn reduce_multiple_orders_with_free_funds(
        &mut self,
        order_packets: Vec<B256>,
    ) -> GoblinResult<()> {
        reduce_multiple_orders::process_reduce_multiple_orders(self, order_packets, None)
    }

    /// Place multiple post-only orders. Used for market making
    ///
    /// # Arguments
    ///
    /// * `to` - Credit posted orders to this trader
    /// * `failed_multiple_limit_order_behavior` - Trade behavior if one of the orders fails
    /// * `bids`
    /// * `asks`
    /// * `client_order_id` - ID provided by trader to uniquely identify this order. It is only emitted
    /// in the event and has no impact on trades. Pass 0 as the default value.
    /// * `use_free_funds` - Whether to use free funds, or transfer new tokens in to place these orders
    ///
    pub fn place_multiple_post_only_orders(
        &mut self,
        to: Address,
        failed_multiple_limit_order_behavior: u8,
        bids: Vec<B256>,
        asks: Vec<B256>,
        client_order_id: u128,
        use_free_funds: bool,
    ) -> GoblinResult<()> {
        new_order::process_multiple_new_orders(
            to,
            FailedMultipleLimitOrderBehavior::decode(failed_multiple_limit_order_behavior)?,
            bids,
            asks,
            client_order_id,
            use_free_funds,
        )
    }

    // TODO how to return struct? Facing AbiType trait error
    pub fn trader_state(&self, trader: Address) -> B256 {
        let slot_storage = SlotStorage::new();
        let trader_state = TraderState::read_from_slot(&slot_storage, trader);

        B256::from_slice(&trader_state.encode())
    }
}
