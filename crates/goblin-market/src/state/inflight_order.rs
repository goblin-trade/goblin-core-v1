use crate::quantities::{AdjustedQuoteLots, BaseLots, QuoteLots, Ticks};

use super::{SelfTradeBehavior, Side};

#[derive(Copy, Clone)]
pub struct InflightOrder {
    pub side: Side,

    pub self_trade_behavior: SelfTradeBehavior,

    /// This is the most aggressive price than an order can be filled at
    pub limit_price_in_ticks: Ticks,

    /// Number of orders to match against.
    pub match_limit: u64,

    /// Available lots to fill against the order book adjusted for fees. If num_base_lots is not set in the `OrderPacket`,
    /// this will be unbounded
    pub base_lot_budget: BaseLots,

    /// Available adjusted quote lots to fill against the order book adjusted for fees. If `num_quote_lots` is not set
    /// in the OrderPacket, this will be unbounded
    pub adjusted_quote_lot_budget: AdjustedQuoteLots,

    /// Number of lots matched in the trade
    pub matched_base_lots: BaseLots,

    /// Number of adjusted quote lots matched in the trade
    pub matched_adjusted_quote_lots: AdjustedQuoteLots,

    /// Number of quote lots paid in fees
    pub quote_lot_fees: QuoteLots,

    // Whether to track block or unix timestamp
    pub track_block: bool,

    // The last valid block or unix timestamp, depending on the value of
    // track_block. Set value as 0 to disable FOK.
    pub last_valid_block_or_unix_timestamp_in_seconds: u32,
}

impl InflightOrder {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        side: Side,
        self_trade_behavior: SelfTradeBehavior,
        limit_price_in_ticks: Ticks,
        match_limit: u64,
        base_lot_budget: BaseLots,
        adjusted_quote_lot_budget: AdjustedQuoteLots,
        track_block: bool,
        last_valid_block_or_unix_timestamp_in_seconds: u32,
    ) -> Self {
        InflightOrder {
            side,
            self_trade_behavior,
            limit_price_in_ticks,
            match_limit,
            base_lot_budget,
            adjusted_quote_lot_budget,
            matched_adjusted_quote_lots: AdjustedQuoteLots::ZERO,
            matched_base_lots: BaseLots::ZERO,
            quote_lot_fees: QuoteLots::ZERO,
            track_block,
            last_valid_block_or_unix_timestamp_in_seconds,
        }
    }

    #[inline(always)]
    pub(crate) fn in_progress(&self) -> bool {
        self.base_lot_budget > BaseLots::ZERO
            && self.adjusted_quote_lot_budget > AdjustedQuoteLots::ZERO
            && self.match_limit > 0
    }

    pub(crate) fn process_match(
        &mut self,
        matched_adjusted_quote_lots: AdjustedQuoteLots,
        matched_base_lots: BaseLots,
    ) {
        if self.match_limit >= 1 {
            self.base_lot_budget -= matched_base_lots;
            self.adjusted_quote_lot_budget -= matched_adjusted_quote_lots;
            self.matched_base_lots += matched_base_lots;
            self.matched_adjusted_quote_lots += matched_adjusted_quote_lots;
            self.match_limit -= 1;
        }
    }
}
