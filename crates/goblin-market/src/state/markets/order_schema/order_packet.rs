use crate::{
    quantities::{BaseLots, QuoteLots, Ticks},
    state::{SelfTradeBehavior, Side},
};

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

        /// Flag for whether or not to reject the order if it would immediately match or amend it to the best non-crossing price
        /// Default value is true
        reject_post_only: bool,

        /// Flag for whether or not the order should only use funds that are already in the account
        /// Using only deposited funds will allow the trader to pass in less accounts per instruction and
        /// save transaction space as well as compute. This is only for traders who have a seat
        use_only_deposited_funds: bool,

        // Whether to track block or unix timestamp
        track_block: bool,

        // The last valid block or unix timestamp, depending on the value of
        // track_block. Set value as 0 to disable FOK.
        last_valid_block_or_unix_timestamp_in_seconds: u32,

        /// If this is set, the order will fail silently if there are insufficient funds
        fail_silently_on_insufficient_funds: bool,
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

        /// Number of orders to match against. If this is `None` there is no limit
        match_limit: Option<u64>,

        /// Client order id used to identify the order in the response to the client
        client_order_id: u128,

        /// Flag for whether or not the order should only use funds that are already in the account.
        /// Using only deposited funds will allow the trader to pass in less accounts per instruction and
        /// save transaction space as well as compute. This is only for traders who have a seat
        use_only_deposited_funds: bool,

        // Whether to track block or unix timestamp
        track_block: bool,

        // The last valid block or unix timestamp, depending on the value of
        // track_block. Set value as 0 to disable FOK.
        last_valid_block_or_unix_timestamp_in_seconds: u32,

        /// If this is set, the order will fail silently if there are insufficient funds
        fail_silently_on_insufficient_funds: bool,
    },

    /// This order type is used to place an order that will be matched against existing resting orders
    /// If the order matches fewer than `min_lots` lots, it will be cancelled.
    ///
    /// Fill or Kill (FOK) orders are a subset of Immediate or Cancel (IOC) orders where either
    /// the `num_base_lots` is equal to the `min_base_lots_to_fill` of the order, or the `num_quote_lots` is
    /// equal to the `min_quote_lots_to_fill` of the order.
    ImmediateOrCancel {
        side: Side,

        /// The most aggressive price an order can be matched at. For example, if there is an IOC buy order
        /// to purchase 10 lots with the tick_per_lot parameter set to 10, then the order will never
        /// be matched at a price higher than 10 quote ticks per base unit. If this value is None, then the order
        /// is treated as a market order.
        price_in_ticks: Option<Ticks>,

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

        /// Number of orders to match against. If set to `None`, there is no limit.
        match_limit: Option<u64>,

        /// Client order id used to identify the order in the program's inner instruction data.
        client_order_id: u128,

        /// Flag for whether or not the order should only use funds that are already in the account.
        /// Using only deposited funds will allow the trader to pass in less accounts per instruction and
        /// save transaction space as well as compute. This is only for traders who have a seat
        use_only_deposited_funds: bool,

        // Whether to track block or unix timestamp
        track_block: bool,

        // The last valid block or unix timestamp, depending on the value of
        // track_block. Set value as 0 to disable FOK.
        last_valid_block_or_unix_timestamp_in_seconds: u32,
    },
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
            Self::Limit { match_limit, .. } => match_limit.unwrap_or(u64::MAX),
            Self::ImmediateOrCancel { match_limit, .. } => match_limit.unwrap_or(u64::MAX),
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

    pub fn get_price_in_ticks(&self) -> Ticks {
        match self {
            Self::PostOnly { price_in_ticks, .. } => *price_in_ticks,
            Self::Limit { price_in_ticks, .. } => *price_in_ticks,
            Self::ImmediateOrCancel { price_in_ticks, .. } => {
                price_in_ticks.unwrap_or(match self.side() {
                    Side::Bid => Ticks::MAX,
                    Side::Ask => Ticks::MIN,
                })
            }
        }
    }
}
