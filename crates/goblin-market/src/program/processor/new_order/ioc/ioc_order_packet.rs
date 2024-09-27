use stylus_sdk::alloy_primitives::Address;

use crate::{
    program::ExpiryChecker,
    quantities::{BaseLots, QuoteLots, Ticks},
    state::{ArbContext, SelfTradeBehavior, Side},
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
    pub fn base_lot_budget(&self) -> BaseLots {
        let base_lots = self.num_base_lots;
        if base_lots == BaseLots::ZERO {
            BaseLots::MAX
        } else {
            base_lots
        }
    }

    pub fn quote_lot_budget(&self) -> Option<QuoteLots> {
        let quote_lots = self.num_quote_lots;
        if quote_lots == QuoteLots::ZERO {
            None
        } else {
            Some(quote_lots)
        }
    }

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
}
