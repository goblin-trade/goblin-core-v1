use stylus_sdk::alloy_primitives::Address;

use crate::{
    parameters::{BASE_LOTS_PER_BASE_UNIT, TICK_SIZE_IN_QUOTE_LOTS_PER_BASE_UNIT},
    program::{
        adjusted_quote_lot_budget_post_fee_adjustment_for_buys_deprecated,
        adjusted_quote_lot_budget_post_fee_adjustment_for_sells_deprecated, compute_quote_lots,
        get_available_base_lots, get_available_quote_lots,
    },
    quantities::{AdjustedQuoteLots, BaseLots, QuoteLots, Ticks},
    state::{order::resting_order::SlotRestingOrder, InflightOrder, SelfTradeBehavior, Side},
    GoblinMarket,
};

pub trait OrderPacketMetadata {
    fn is_take_only(&self) -> bool {
        self.is_ioc() || self.is_fok()
    }
    fn is_ioc(&self) -> bool;
    fn is_fok(&self) -> bool;
    fn is_limit(&self) -> bool;
    fn is_post_only(&self) -> bool;
    fn no_deposit_or_withdrawal(&self) -> bool;
}

#[derive(Copy, Clone)]
pub enum OrderPacket {
    /// This order type is used to place a limit order on the book.
    /// It will never be matched against other existing limit orders
    PostOnly {
        side: Side,

        /// The price of the order, in ticks
        price_in_ticks: Ticks,

        /// Number of base lots to place on the book
        num_base_lots: BaseLots,

        /// Client order id used to identify the order in the response to the client
        client_order_id: u128,

        /// Whether to fail the order if it crosses, or to amend it to the best non-crossing price
        fail_on_cross: bool,

        /// Flag for whether or not the order should only use funds that are already
        /// credited in the trader state. This saves gas.
        use_only_deposited_funds: bool,

        // Whether to track block or unix timestamp
        track_block: bool,

        // The last valid block or unix timestamp, depending on the value of
        // track_block. Set value as 0 to disable FOK.
        last_valid_block_or_unix_timestamp_in_seconds: u32,

        /// If this is set, the order will fail silently if there are insufficient funds
        fail_silently_on_insufficient_funds: bool,

        /// Specifies the number of ticks to adjust the price when the current price level (price_on_ticks)
        /// has no available slots. This adjustment moves the order to a less aggressive price, further
        /// away from the market center, in an attempt to find an available slot.
        tick_offset: u8,
    },

    /// This order type is used to place a limit order on the book
    /// It can be matched against other existing limit orders, but will posted at the
    /// specified level if it is not matched
    Limit {
        side: Side,

        /// The price of the order, in ticks
        price_in_ticks: Ticks,

        /// Total number of base lots to place on the book or fill at a better price
        num_base_lots: BaseLots,

        /// How the matching engine should handle a self trade
        self_trade_behavior: SelfTradeBehavior,

        /// Number of orders to match against. Pass u64::MAX to have no limit (this is the default)
        match_limit: u64,

        /// Client order id used to identify the order in the response to the client
        client_order_id: u128,

        /// Flag for whether or not the order should only use funds that are already
        /// credited in the trader state. This saves gas.
        use_only_deposited_funds: bool,

        // Whether to track block or unix timestamp
        track_block: bool,

        // The last valid block or unix timestamp, depending on the value of
        // track_block. Set value as 0 to disable FOK.
        last_valid_block_or_unix_timestamp_in_seconds: u32,

        /// If this is set, the order will fail silently if there are insufficient funds
        fail_silently_on_insufficient_funds: bool,

        /// Specifies the number of ticks to adjust the price when the current price level (price_on_ticks)
        /// has no available slots. This adjustment moves the order to a less aggressive price, further
        /// away from the market center, in an attempt to find an available slot.
        tick_offset: u8,
    },

    /// This order type is used to place an order that will be matched against existing resting orders
    /// If the order matches fewer than `min_lots` lots, it will be cancelled.
    ///
    /// Fill or Kill (FOK) orders are a subset of Immediate or Cancel (IOC) orders where either
    /// the `num_base_lots` is equal to the `min_base_lots_to_fill` of the order, or the `num_quote_lots` is
    /// equal to the `min_quote_lots_to_fill` of the order.
    ImmediateOrCancel {
        side: Side,

        /// The most aggressive (worst) price an order can be matched at. For example, if there is an IOC buy order
        /// to purchase 10 lots with the tick_per_lot parameter set to 10, then the order will never
        /// be matched at a price higher than 10 quote ticks per base unit.
        /// To run a market order without price limit, pass u64::MAX for bids and 0 for asks.
        price_in_ticks: Ticks,

        /// The number of base lots to fill against the order book. Either this parameter or the `num_quote_lots`
        /// parameter must be set to a nonzero value.
        num_base_lots: BaseLots,

        /// The number of quote lots to fill against the order book. Either this parameter or the `num_base_lots`
        /// parameter must be set to a nonzero value.
        num_quote_lots: QuoteLots,

        /// The minimum number of base lots to fill against the order book. If the order does not fill
        /// this many base lots, it will be voided.
        min_base_lots_to_fill: BaseLots,

        /// The minimum number of quote lots to fill against the order book. If the order does not fill
        /// this many quote lots, it will be voided.
        min_quote_lots_to_fill: QuoteLots,

        /// How the matching engine should handle a self trade.
        self_trade_behavior: SelfTradeBehavior,

        /// Number of orders to match against. Pass u64::MAX to have no limit (this is the default)
        match_limit: u64,

        /// Client order id used to identify the order in the response to the client
        client_order_id: u128,

        /// Flag for whether or not the order should only use funds that are already
        /// credited in the trader state. This saves gas.
        use_only_deposited_funds: bool,

        // Whether to track block or unix timestamp
        track_block: bool,

        // The last valid block or unix timestamp, depending on the value of
        // track_block. Set value as 0 to disable FOK.
        last_valid_block_or_unix_timestamp_in_seconds: u32,
    },
}

impl OrderPacketMetadata for OrderPacket {
    fn is_ioc(&self) -> bool {
        matches!(self, OrderPacket::ImmediateOrCancel { .. })
    }

    fn is_fok(&self) -> bool {
        match self {
            &Self::ImmediateOrCancel {
                num_base_lots,
                num_quote_lots,
                min_base_lots_to_fill,
                min_quote_lots_to_fill,
                ..
            } => {
                num_base_lots > BaseLots::ZERO && num_base_lots == min_base_lots_to_fill
                    || num_quote_lots > QuoteLots::ZERO && num_quote_lots == min_quote_lots_to_fill
            }
            _ => false,
        }
    }

    fn is_post_only(&self) -> bool {
        matches!(self, OrderPacket::PostOnly { .. })
    }

    fn is_limit(&self) -> bool {
        matches!(self, OrderPacket::Limit { .. })
    }

    fn no_deposit_or_withdrawal(&self) -> bool {
        match *self {
            Self::PostOnly {
                use_only_deposited_funds,
                ..
            } => use_only_deposited_funds,
            Self::Limit {
                use_only_deposited_funds,
                ..
            } => use_only_deposited_funds,
            Self::ImmediateOrCancel {
                use_only_deposited_funds,
                ..
            } => use_only_deposited_funds,
        }
    }
}

impl OrderPacket {
    pub fn side(&self) -> Side {
        match self {
            Self::PostOnly { side, .. } => *side,
            Self::Limit { side, .. } => *side,
            Self::ImmediateOrCancel { side, .. } => *side,
        }
    }

    pub fn fail_silently_on_insufficient_funds(&self) -> bool {
        match self {
            Self::PostOnly {
                fail_silently_on_insufficient_funds,
                ..
            } => *fail_silently_on_insufficient_funds,
            Self::Limit {
                fail_silently_on_insufficient_funds,
                ..
            } => *fail_silently_on_insufficient_funds,
            Self::ImmediateOrCancel { .. } => false,
        }
    }

    pub fn client_order_id(&self) -> u128 {
        match self {
            Self::PostOnly {
                client_order_id, ..
            } => *client_order_id,
            Self::Limit {
                client_order_id, ..
            } => *client_order_id,
            Self::ImmediateOrCancel {
                client_order_id, ..
            } => *client_order_id,
        }
    }

    pub fn num_base_lots(&self) -> BaseLots {
        match self {
            Self::PostOnly { num_base_lots, .. } => *num_base_lots,
            Self::Limit { num_base_lots, .. } => *num_base_lots,
            Self::ImmediateOrCancel { num_base_lots, .. } => *num_base_lots,
        }
    }

    pub fn num_quote_lots(&self) -> QuoteLots {
        match self {
            Self::PostOnly { .. } => QuoteLots::ZERO,
            Self::Limit { .. } => QuoteLots::ZERO,
            Self::ImmediateOrCancel { num_quote_lots, .. } => *num_quote_lots,
        }
    }

    pub fn base_lot_budget(&self) -> BaseLots {
        let base_lots = self.num_base_lots();
        if base_lots == BaseLots::ZERO {
            BaseLots::MAX
        } else {
            base_lots
        }
    }

    pub fn quote_lot_budget(&self) -> Option<QuoteLots> {
        let quote_lots = self.num_quote_lots();
        if quote_lots == QuoteLots::ZERO {
            None
        } else {
            Some(quote_lots)
        }
    }

    pub fn match_limit(&self) -> u64 {
        match self {
            Self::PostOnly { .. } => u64::MAX,
            Self::Limit { match_limit, .. } => *match_limit,
            Self::ImmediateOrCancel { match_limit, .. } => *match_limit,
        }
    }

    pub fn self_trade_behavior(&self) -> SelfTradeBehavior {
        match self {
            Self::PostOnly { .. } => panic!("PostOnly orders do not have a self trade behavior"),
            Self::Limit {
                self_trade_behavior,
                ..
            } => *self_trade_behavior,
            Self::ImmediateOrCancel {
                self_trade_behavior,
                ..
            } => *self_trade_behavior,
        }
    }

    pub fn tick_offset(&self) -> u8 {
        match self {
            Self::PostOnly { tick_offset, .. } => *tick_offset,
            Self::Limit { tick_offset, .. } => *tick_offset,
            Self::ImmediateOrCancel { .. } => {
                panic!("ImmediateOrCancel orders do not have tick_offset field")
            }
        }
    }

    pub fn get_price_in_ticks(&self) -> Ticks {
        match self {
            Self::PostOnly { price_in_ticks, .. } => *price_in_ticks,
            Self::Limit { price_in_ticks, .. } => *price_in_ticks,
            Self::ImmediateOrCancel { price_in_ticks, .. } => *price_in_ticks,
        }
    }

    pub fn set_price_in_ticks(&mut self, price_in_ticks: Ticks) {
        match self {
            Self::PostOnly {
                price_in_ticks: old_price_in_ticks,
                ..
            } => *old_price_in_ticks = price_in_ticks,
            Self::Limit {
                price_in_ticks: old_price_in_ticks,
                ..
            } => *old_price_in_ticks = price_in_ticks,
            Self::ImmediateOrCancel {
                price_in_ticks: old_price_in_ticks,
                ..
            } => *old_price_in_ticks = price_in_ticks,
        }
    }

    pub fn track_block(&self) -> bool {
        match self {
            Self::PostOnly { track_block, .. } => *track_block,
            Self::Limit { track_block, .. } => *track_block,
            Self::ImmediateOrCancel { track_block, .. } => *track_block,
        }
    }

    pub fn last_valid_block_or_unix_timestamp_in_seconds(&self) -> u32 {
        match self {
            Self::PostOnly {
                last_valid_block_or_unix_timestamp_in_seconds,
                ..
            } => *last_valid_block_or_unix_timestamp_in_seconds,
            Self::Limit {
                last_valid_block_or_unix_timestamp_in_seconds,
                ..
            } => *last_valid_block_or_unix_timestamp_in_seconds,
            Self::ImmediateOrCancel {
                last_valid_block_or_unix_timestamp_in_seconds,
                ..
            } => *last_valid_block_or_unix_timestamp_in_seconds,
        }
    }

    pub fn get_last_valid_block(&self) -> Option<u32> {
        match self {
            Self::PostOnly {
                track_block,
                last_valid_block_or_unix_timestamp_in_seconds,
                ..
            } => get_last_valid_block(*track_block, *last_valid_block_or_unix_timestamp_in_seconds),
            Self::Limit {
                track_block,
                last_valid_block_or_unix_timestamp_in_seconds,
                ..
            } => get_last_valid_block(*track_block, *last_valid_block_or_unix_timestamp_in_seconds),
            Self::ImmediateOrCancel {
                track_block,
                last_valid_block_or_unix_timestamp_in_seconds,
                ..
            } => get_last_valid_block(*track_block, *last_valid_block_or_unix_timestamp_in_seconds),
        }
    }

    pub fn get_last_valid_unix_timestamp_in_seconds(&self) -> Option<u32> {
        match self {
            Self::PostOnly {
                track_block,
                last_valid_block_or_unix_timestamp_in_seconds,
                ..
            } => get_last_valid_unix_timestamp(
                *track_block,
                *last_valid_block_or_unix_timestamp_in_seconds,
            ),
            Self::Limit {
                track_block,
                last_valid_block_or_unix_timestamp_in_seconds,
                ..
            } => get_last_valid_unix_timestamp(
                *track_block,
                *last_valid_block_or_unix_timestamp_in_seconds,
            ),
            Self::ImmediateOrCancel {
                track_block,
                last_valid_block_or_unix_timestamp_in_seconds,
                ..
            } => get_last_valid_unix_timestamp(
                *track_block,
                *last_valid_block_or_unix_timestamp_in_seconds,
            ),
        }
    }

    pub fn is_expired(&self, current_block: u32, current_unix_timestamp_in_seconds: u32) -> bool {
        if let Some(last_valid_block) = self.get_last_valid_block() {
            if current_block > last_valid_block {
                return true;
            }
        }
        if let Some(last_valid_unix_timestamp_in_seconds) =
            self.get_last_valid_unix_timestamp_in_seconds()
        {
            if current_unix_timestamp_in_seconds > last_valid_unix_timestamp_in_seconds {
                return true;
            }
        }
        false
    }

    // pub fn has_sufficient_funds(
    //     &self,
    //     context: &GoblinMarket,
    //     trader: Address,
    //     base_lots_available: &mut BaseLots,
    //     quote_lots_available: &mut QuoteLots,
    //     base_allowance_read: &mut bool,
    //     quote_allowance_read: &mut bool,
    // ) -> bool {
    //     match self.side() {
    //         Side::Ask => {
    //             if *base_lots_available < self.num_base_lots() {
    //                 // Lazy load available approved balance for base token
    //                 if !*base_allowance_read {
    //                     *base_lots_available += get_available_base_lots(context, trader);
    //                     *base_allowance_read = true;
    //                 }

    //                 return *base_lots_available >= self.num_base_lots();
    //             }
    //         }
    //         Side::Bid => {
    //             let quote_lots_required =
    //                 compute_quote_lots(self.get_price_in_ticks(), self.num_base_lots());

    //             if *quote_lots_available < quote_lots_required {
    //                 // Lazy load available approved balance for quote token
    //                 if !*quote_allowance_read {
    //                     *quote_lots_available += get_available_quote_lots(context, trader);

    //                     *quote_allowance_read = true;
    //                 }

    //                 return *quote_lots_available >= quote_lots_required;
    //             }
    //         }
    //     }
    //     true
    // }

    /// The adjusted quote lot budget
    ///
    /// Multiply the quote lot budget by the number of base lots per unit to get the number of
    /// adjusted quote lots (quote_lots * base_lots_per_base_unit).
    ///
    /// Externally ensure this is not called for post-only orders.
    ///
    /// This function is only called for taker orders (IOC and limit). Since quote_lot_budget()
    /// is None for limit orders, this function will return AdjustedQuoteLots::MAX for limit.
    pub fn adjusted_quote_lot_budget(&self) -> AdjustedQuoteLots {
        debug_assert_eq!(self.is_post_only(), false);

        if let Some(quote_lot_budget) = self.quote_lot_budget() {
            let size_in_adjusted_quote_lots = quote_lot_budget * BASE_LOTS_PER_BASE_UNIT;

            match self.side() {
                // For buys, the adjusted quote lot budget is decreased by the max fee.
                // This is because the fee is added to the quote lots spent after the matching is complete.
                Side::Bid => adjusted_quote_lot_budget_post_fee_adjustment_for_buys_deprecated(
                    size_in_adjusted_quote_lots,
                ),
                // For sells, the adjusted quote lot budget is increased by the max fee.
                // This is because the fee is subtracted from the quote lot received after the matching is complete.
                Side::Ask => adjusted_quote_lot_budget_post_fee_adjustment_for_sells_deprecated(
                    size_in_adjusted_quote_lots,
                ),
            }
            .unwrap_or(AdjustedQuoteLots::MAX)
        } else {
            AdjustedQuoteLots::MAX
        }
    }

    /// Generate inflight order from a limit or IOC order packet
    ///
    /// Externally ensure this is not called for post-only orders.
    pub fn get_inflight_order(&self) -> InflightOrder {
        debug_assert_eq!(self.is_post_only(), false);

        InflightOrder::new(
            self.side(),
            self.self_trade_behavior(),
            self.get_price_in_ticks(),
            self.match_limit(),
            self.base_lot_budget(),
            self.adjusted_quote_lot_budget(),
            self.track_block(),
            self.last_valid_block_or_unix_timestamp_in_seconds(),
        )
    }

    /// Generate resting order for a post-only order packet
    ///
    /// Externally ensure this is only called for post-only orders.
    ///
    /// # Arguments
    ///
    /// * `trader_address` - Address of the trader posting the order
    pub fn get_resting_order(&self, trader_address: Address) -> SlotRestingOrder {
        debug_assert_eq!(self.is_post_only(), true);

        SlotRestingOrder::new(
            trader_address,
            self.num_base_lots(),
            self.track_block(),
            self.last_valid_block_or_unix_timestamp_in_seconds(),
        )
    }
}

fn get_last_valid_block(
    track_block: bool,
    last_valid_block_or_unix_timestamp_in_seconds: u32,
) -> Option<u32> {
    if !track_block || last_valid_block_or_unix_timestamp_in_seconds == 0 {
        None
    } else {
        Some(last_valid_block_or_unix_timestamp_in_seconds)
    }
}

fn get_last_valid_unix_timestamp(
    track_block: bool,
    last_valid_block_or_unix_timestamp_in_seconds: u32,
) -> Option<u32> {
    if track_block || last_valid_block_or_unix_timestamp_in_seconds == 0 {
        None
    } else {
        Some(last_valid_block_or_unix_timestamp_in_seconds)
    }
}
