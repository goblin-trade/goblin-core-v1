use stylus_sdk::alloy_primitives::Address;

use crate::{
    parameters::BASE_LOTS_PER_BASE_UNIT,
    program::{
        adjusted_quote_lot_budget_post_fee_adjustment_for_buys_deprecated,
        adjusted_quote_lot_budget_post_fee_adjustment_for_sells_deprecated,
        compute_adjusted_quote_lots, ExpiryChecker,
    },
    quantities::{AdjustedQuoteLots, BaseLots, QuoteLots, Ticks},
    state::{ArbContext, InflightOrder, SelfTradeBehavior, Side},
};

pub struct ImmediateOrCancelOrderPacket {
    pub side: Side,

    /// The trader address
    pub trader: Address,

    /// The most aggressive (worst) price an order can be matched at. For example, if there is an IOC buy order
    /// to purchase 10 lots with the tick_per_lot parameter set to 10, then the order will never
    /// be matched at a price higher than 10 quote ticks per base unit.
    /// To run a market order without price limit, pass u64::MAX for bids and 0 for asks.
    pub price_in_ticks: Ticks,

    /// The number of base lots to fill against the order book. Either this parameter or the `num_quote_lots`
    /// parameter must be set to a nonzero value.
    pub num_base_lots: BaseLots,

    /// The number of quote lots to fill against the order book. Either this parameter or the `num_base_lots`
    /// parameter must be set to a nonzero value.
    pub num_quote_lots: QuoteLots,

    /// The minimum number of base lots to fill against the order book. If the order does not fill
    /// this many base lots, it will be voided.
    pub min_base_lots_to_fill: BaseLots,

    /// The minimum number of quote lots to fill against the order book. If the order does not fill
    /// this many quote lots, it will be voided.
    pub min_quote_lots_to_fill: QuoteLots,

    /// How the matching engine should handle a self trade.
    pub self_trade_behavior: SelfTradeBehavior,

    /// Number of orders to match against. Pass u64::MAX to have no limit (this is the default)
    pub match_limit: u64,

    /// Flag for whether or not the order should only use funds that are already
    /// credited in the trader state. This saves gas.
    pub use_only_deposited_funds: bool,

    // Whether to track block or unix timestamp
    pub track_block: bool,

    // The last valid block or unix timestamp, depending on the value of
    // track_block. Set value as 0 to disable FOK.
    pub last_valid_block_or_unix_timestamp_in_seconds: u32,
}

impl ImmediateOrCancelOrderPacket {
    pub fn set_price_in_ticks(&mut self, price_in_ticks: Ticks) {
        self.price_in_ticks = price_in_ticks;
    }

    pub fn is_invalid(&self, ctx: &ArbContext, expiry_checker: &mut ExpiryChecker) -> bool {
        // Validate price
        self.side == Side::Bid && self.price_in_ticks == Ticks::ZERO

            // Validate lots- One must be zero and the other non-zero
            || self.num_base_lots == BaseLots::ZERO && self.num_quote_lots == QuoteLots::ZERO
            || self.num_base_lots > BaseLots::ZERO && self.num_quote_lots > QuoteLots::ZERO

            // Validate expiry block number / timestamp
            || expiry_checker.is_expired(
                ctx,
                self.track_block,
                self.last_valid_block_or_unix_timestamp_in_seconds,
            )
    }

    /// Get the base lot budget. If the number of base lots is zero (bid case)
    /// then the budget is set to max.
    // pub fn base_lot_budget(&self) -> BaseLots {
    //     let base_lots = self.num_base_lots;
    //     // TODO Why do 0 base lots map to MAX?
    //     if base_lots == BaseLots::ZERO {
    //         // Bid case
    //         BaseLots::MAX
    //     } else {
    //         // Ask case
    //         base_lots
    //     }
    // }

    pub fn base_lot_budget_v2(&self) -> BaseLots {
        // TODO check limit orders. Does base_lots == BaseLots::ZERO apply for
        // asks in limit orders?
        match self.side {
            // Bid IOC orders have num_base_lots = 0. Map it to MAX
            Side::Bid => BaseLots::MAX,
            Side::Ask => self.num_base_lots,
        }
    }

    // Remove if not used elsehere. Usage skipped in adjusted_quote_lot_budget()
    // /// Get the quote lot budget. If the number of quote lots is zero (ask case)
    // /// then the budget is set to max.
    // pub fn quote_lot_budget(&self) -> Option<QuoteLots> {
    //     let quote_lots = self.num_quote_lots;
    //     if quote_lots == QuoteLots::ZERO {
    //         None
    //     } else {
    //         Some(quote_lots)
    //     }
    // }

    /// The adjusted quote lot budget (quote lots * base lots / base lot size)
    ///
    /// If num_quote_lots are zero (ask case) then budget is set to max.
    // pub fn adjusted_quote_lot_budget(&self) -> AdjustedQuoteLots {
    //     if self.num_quote_lots == QuoteLots::ZERO {
    //         // Ask case
    //         AdjustedQuoteLots::MAX
    //     } else {
    //         // Bid case
    //         compute_adjusted_quote_lots(self.side, self.num_quote_lots)
    //     }
    // }

    pub fn adjusted_quote_lot_budget_v2(&self) -> AdjustedQuoteLots {
        match self.side {
            // compute_adjusted_quote_lots() has an Ask branch. It is possible
            // that Asks in limit orders have self.num_quote_lots == QuoteLots::ZERO
            Side::Bid => compute_adjusted_quote_lots(self.side, self.num_quote_lots),
            // Ask IOC orders have num_quote_lots = 0. Map it to MAX
            Side::Ask => AdjustedQuoteLots::MAX,
        }
    }

    pub fn get_inflight_order(&self) -> InflightOrder {
        // Retain v1 functions. Perhaps limit orders have a different condition
        // when finding base_lot_budget() and adjusted_quote_lot_budget()
        InflightOrder::new(
            self.side,
            self.self_trade_behavior,
            self.price_in_ticks,
            self.match_limit,
            self.base_lot_budget_v2(),
            self.adjusted_quote_lot_budget_v2(),
            self.track_block,
            self.last_valid_block_or_unix_timestamp_in_seconds,
        )
    }
}
