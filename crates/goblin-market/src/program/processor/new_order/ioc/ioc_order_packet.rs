use crate::{
    quantities::{BaseLots, QuoteLots, Ticks},
    state::{SelfTradeBehavior, Side},
};

pub struct ImmediateOrCancelOrderPacket {
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

    // pub fn get_last_valid_block(&self) {
    //     get_last_valid_block(*track_block, *last_valid_block_or_unix_timestamp_in_seconds)
    // }
}
