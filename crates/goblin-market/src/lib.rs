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
    place_multiple_new_orders, process_new_order,
    processor::{deposit, fees, withdraw},
    reduce_multiple_orders, FailedMultipleLimitOrderBehavior,
};
use quantities::{BaseLots, Ticks, WrapperU64};
use state::{OrderPacket, SelfTradeBehavior, Side, SlotActions, SlotStorage, TraderState};
use stylus_sdk::{
    alloy_primitives::{Address, FixedBytes, B256},
    msg,
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
    /// * `bids`
    /// * `asks`
    /// * `failed_multiple_limit_order_behavior` - Trade behavior if one of the orders fails
    /// * `tick_offset` - Adjust the price by given number of ticks if there are no slots available
    /// at current price. The entire TX fails if a single resting order can't be offsetted.
    /// * `client_order_id` - ID provided by trader to uniquely identify this order. It is only emitted
    /// in the event and has no impact on trades. Pass 0 as the default value.
    /// * `use_free_funds` - Whether to use free funds, or transfer new tokens in to place these orders
    /// * `to` - Credit posted orders to this trader
    pub fn place_multiple_post_only_orders(
        &mut self,
        bids: Vec<FixedBytes<21>>,
        asks: Vec<FixedBytes<21>>,
        failed_multiple_limit_order_behavior: u8,
        tick_offset: u8,
        client_order_id: u128,
        use_free_funds: bool,
        to: Address,
    ) -> GoblinResult<()> {
        place_multiple_new_orders(
            self,
            msg::sender(),
            to,
            FailedMultipleLimitOrderBehavior::from(failed_multiple_limit_order_behavior),
            bids,
            asks,
            client_order_id,
            use_free_funds,
            tick_offset,
        )
    }

    /// Place a limit order on the book
    ///
    /// # Arguments
    ///
    pub fn place_limit_order(
        &mut self,
        is_bid: bool,
        price_in_ticks: u64,
        num_base_lots: u64,
        self_trade_behavior: u8,
        match_limit: u64,
        client_order_id: u128,
        track_block: bool,
        last_valid_block_or_unix_timestamp_in_seconds: u32,
        fail_silently_on_insufficient_funds: bool,
        tick_offset: u8,
    ) -> GoblinResult<()> {
        let mut order_packet = OrderPacket::Limit {
            side: Side::from(is_bid),
            price_in_ticks: Ticks::new(price_in_ticks),
            num_base_lots: BaseLots::new(num_base_lots),
            self_trade_behavior: SelfTradeBehavior::from(self_trade_behavior),
            match_limit,
            client_order_id,
            use_only_deposited_funds: false,
            track_block,
            last_valid_block_or_unix_timestamp_in_seconds,
            fail_silently_on_insufficient_funds,
            tick_offset,
        };

        process_new_order(self, &mut order_packet, msg::sender())
    }

    // TODO how to return struct? Facing AbiType trait error
    pub fn trader_state(&self, trader: Address) -> B256 {
        let slot_storage = SlotStorage::new();
        let trader_state = TraderState::read_from_slot(&slot_storage, trader);

        B256::from_slice(&trader_state.encode())
    }
}
