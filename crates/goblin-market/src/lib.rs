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
    reduce_multiple_orders,
};
use quantities::{BaseLots, QuoteLots, Ticks, WrapperU64};
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
    /// Deposit funds into the trader account. These funds can be used to trade
    /// with lower gas costs because ERC20 transfers are avoided.
    ///
    /// A wallet can credit funds to another trader.
    ///
    /// # Arguments
    ///
    /// * `trader` - Credit funds to this trader. A wallet can credit funds to another trader.
    /// * `quote_lots`
    /// * `base_lots`
    ///
    pub fn deposit_funds(
        &mut self,
        trader: Address,
        quote_lots: u64,
        base_lots: u64,
    ) -> GoblinResult<()> {
        deposit::process_deposit_funds(
            self,
            trader,
            QuoteLots::new(quote_lots),
            BaseLots::new(base_lots),
        )
    }

    /// Withdraw free funds for a given trader
    ///
    /// # Arguments
    ///
    /// * `trader` - Withdraw funds from this trader
    /// * `recipient` - Credit to this wallet
    /// * `num_quote_lots` - Number of lots to withdraw. Pass 0 if none should be withdrawn.
    /// Pass U64::MAX to withdraw all.
    /// * `num_base_lots` - Number of lots to withdraw. Pass 0 if none should be withdrawn.
    /// Pass U64::MAX to withdraw all.
    ///
    pub fn withdraw_funds(
        &mut self,
        recipient: Address,
        quote_lots: u64,
        base_lots: u64,
    ) -> GoblinResult<()> {
        withdraw::process_withdraw_funds(
            self,
            msg::sender(),
            recipient,
            QuoteLots::new(quote_lots),
            BaseLots::new(base_lots),
        )
    }

    /// Collect protocol fees
    ///
    /// Only callable by the FEE_COLLECTOR
    ///
    /// # Parameters
    ///
    /// * `recipient` - Transfer fees to this address
    ///
    pub fn collect_fees(&mut self, recipient: Address) -> GoblinResult<()> {
        fees::process_collect_fees(self, msg::sender(), recipient)
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
    ///
    pub fn place_multiple_post_only_orders(
        &mut self,
        bids: Vec<FixedBytes<21>>,
        asks: Vec<FixedBytes<21>>,
        fail_on_cross: bool,
        skip_on_insufficient_funds: bool,
        tick_offset: u8,
        client_order_id: u128,
        use_free_funds: bool,
    ) -> GoblinResult<()> {
        place_multiple_new_orders(
            self,
            bids,
            asks,
            msg::sender(),
            fail_on_cross,
            skip_on_insufficient_funds,
            client_order_id,
            use_free_funds,
            tick_offset,
        )
    }

    /// Place an IOC order. This is also known as a taker order and swap.
    ///
    /// # Arguments
    ///
    /// * `is_bid`- Whether a bid or an ask
    /// * `price_in_ticks` - The worst price against which the order can be matched against.
    /// Matching stops if this price is reached. To run a market order without price limit,
    /// pass u64::MAX for bids and 0 for asks.
    /// * `num_lots_in` - Number of lots to be traded in.
    /// * `min_lots_to_fill` - Minimum output lots to be filled.
    /// * `self_trade_behavior` - How the matching engine should handle a self trade.
    /// * `match_limit` - Number of orders to match against.
    /// Pass u64::MAX to have no limit (this is the default)
    /// * `client_order_id` - Client order id used to identify the order in the response to the client
    /// * `track_block`
    /// * `last_valid_block_or_unix_timestamp_in_seconds`
    /// * `use_only_deposited_funds` - Whether to only use trader funds deposited with
    /// the exchange, or whether to transfer in new tokens.
    ///
    pub fn place_ioc_order(
        &mut self,
        is_bid: bool,
        price_in_ticks: u64,
        num_lots_in: u64,
        min_lots_to_fill: u64,
        self_trade_behavior: u8,
        match_limit: u64,
        client_order_id: u128,
        track_block: bool,
        last_valid_block_or_unix_timestamp_in_seconds: u32,
        use_only_deposited_funds: bool,
    ) -> GoblinResult<()> {
        let (num_base_lots, num_quote_lots, min_base_lots_to_fill, min_quote_lots_to_fill) =
            if is_bid {
                // bid (buy)- quote token in, base token out
                (
                    BaseLots::ZERO,
                    QuoteLots::new(num_lots_in),
                    BaseLots::new(min_lots_to_fill),
                    QuoteLots::ZERO,
                )
            } else {
                (
                    BaseLots::new(num_lots_in),
                    QuoteLots::ZERO,
                    BaseLots::ZERO,
                    QuoteLots::new(min_lots_to_fill),
                )
            };

        let mut order_packet = OrderPacket::ImmediateOrCancel {
            side: Side::from(is_bid),
            price_in_ticks: Ticks::new(price_in_ticks),
            num_base_lots,
            num_quote_lots,
            min_base_lots_to_fill,
            min_quote_lots_to_fill,
            self_trade_behavior: SelfTradeBehavior::from(self_trade_behavior),
            match_limit,
            client_order_id,
            use_only_deposited_funds,
            track_block,
            last_valid_block_or_unix_timestamp_in_seconds,
        };

        process_new_order(self, &mut order_packet, msg::sender())
    }

    /// Place a limit order on the book
    ///
    /// # Arguments
    ///
    /// * `is_bid`- Whether a bid or an ask
    /// * `price_in_ticks`
    /// * `num_base_lots`
    /// * `self_trade_behavior`
    /// * `match_limit`
    /// * `client_order_id`
    /// * `track_block`
    /// * `last_valid_block_or_unix_timestamp_in_seconds`
    /// * `fail_silently_on_insufficient_funds`
    /// * `tick_offset`
    /// * `use_only_deposited_funds` - Whether to only use trader funds deposited with
    /// the exchange, or whether to transfer in new tokens.
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
        use_only_deposited_funds: bool,
    ) -> GoblinResult<()> {
        let mut order_packet = OrderPacket::Limit {
            side: Side::from(is_bid),
            price_in_ticks: Ticks::new(price_in_ticks),
            num_base_lots: BaseLots::new(num_base_lots),
            self_trade_behavior: SelfTradeBehavior::from(self_trade_behavior),
            match_limit,
            client_order_id,
            use_only_deposited_funds,
            track_block,
            last_valid_block_or_unix_timestamp_in_seconds,
            fail_silently_on_insufficient_funds,
            tick_offset,
        };

        process_new_order(self, &mut order_packet, msg::sender())
    }

    /// Place a limit order on the book
    ///
    /// # Arguments
    ///
    /// * `is_bid`- Whether a bid or an ask
    /// * `price_in_ticks`
    /// * `num_base_lots`
    /// * `fail_on_cross`
    /// * `client_order_id`
    /// * `track_block`
    /// * `last_valid_block_or_unix_timestamp_in_seconds`
    /// * `fail_silently_on_insufficient_funds`
    /// * `tick_offset`
    /// * `use_only_deposited_funds` - Whether to only use trader funds deposited with
    /// the exchange, or whether to transfer in new tokens.
    ///
    pub fn place_post_only_order(
        &mut self,
        is_bid: bool,
        price_in_ticks: u64,
        num_base_lots: u64,
        fail_on_cross: bool,
        client_order_id: u128,
        track_block: bool,
        last_valid_block_or_unix_timestamp_in_seconds: u32,
        fail_silently_on_insufficient_funds: bool,
        tick_offset: u8,
        use_only_deposited_funds: bool,
    ) -> GoblinResult<()> {
        let mut order_packet = OrderPacket::PostOnly {
            side: Side::from(is_bid),
            price_in_ticks: Ticks::new(price_in_ticks),
            num_base_lots: BaseLots::new(num_base_lots),
            client_order_id,
            fail_on_cross,
            use_only_deposited_funds,
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
