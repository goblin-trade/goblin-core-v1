use crate::{
    parameters::{BASE_LOTS_PER_BASE_UNIT, TICK_SIZE_IN_QUOTE_LOTS_PER_BASE_UNIT},
    program::{
        compute_fee, compute_fee_v2, compute_quote_lots_from_adjusted_quote_lots_ceil,
        compute_quote_lots_from_adjusted_quote_lots_floor, round_adjusted_quote_lots_down,
        round_adjusted_quote_lots_up,
    },
    quantities::{AdjustedQuoteLots, BaseLots, QuoteLots, QuoteLotsPerBaseUnit, Ticks},
};

use super::{SelfTradeBehavior, Side};

#[derive(Copy, Clone)]
pub struct InflightOrder {
    pub side: Side,

    pub self_trade_behavior: SelfTradeBehavior,

    /// This is the most aggressive price than an order can be filled at
    pub limit_price_in_ticks: Ticks,

    /// Max number of orders to match against.
    pub match_limit: u64,

    /// Available lots to fill against the order book adjusted for fees. If num_base_lots is not set in the `OrderPacket`,
    /// this will be unbounded
    pub base_lot_budget: BaseLots,

    /// Available adjusted quote lots to fill against the order book adjusted for fees. If `num_quote_lots` is not set
    /// in the OrderPacket, this will be unbounded
    pub adjusted_quote_lot_budget: AdjustedQuoteLots,

    /// Number of lots matched in the trade.
    /// Evaluated against `min_base_lots_to_fill` for minimum fill condition.
    pub matched_base_lots: BaseLots,

    /// Number of adjusted quote lots matched in the trade.
    /// Used to calculate fees and matched_quote_lots(). The latter is evaluated
    /// against `min_quote_lots_to_fill` for minimum fill condition.
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

    /// Process a self trade of type DecrementTake.
    ///
    /// The available budgets and match limit are decremented but matched lots
    /// are not added to matched lots. This is because matched lots are used to calculate fees.
    /// DecrementTake self trades should not produce taker fees.
    ///
    pub(crate) fn process_decrement_take(
        &mut self,
        matched_adjusted_quote_lots: AdjustedQuoteLots,
        matched_base_lots: BaseLots,
    ) {
        debug_assert!(self.match_limit > 0);

        // TODO need saturated sub?
        // - base_lot_budget is guaranteed to be smaller
        // - What about adjusted_quote_lot_budget?
        // adjusted_quote_lot_budget is MAX for quote orders. Subtraction
        // cannot underflow.
        self.base_lot_budget -= matched_base_lots;
        self.adjusted_quote_lot_budget -= matched_adjusted_quote_lots;
        // Self trades will count towards the match limit
        self.match_limit -= 1;
    }

    pub(crate) fn compute_fees(&self) -> QuoteLots {
        let fee_in_adjusted_quote_lots = compute_fee(self.matched_adjusted_quote_lots);
        round_adjusted_quote_lots_up(fee_in_adjusted_quote_lots) / BASE_LOTS_PER_BASE_UNIT
    }

    /// Compute fees in quote lots from matched_adjusted_quote_lots
    ///
    /// * fees = fee_in_adjusted_quote_lots.div_ceil(BASE_LOTS_PER_BASE_UNIT)
    /// * fees should be a multiple of BASE_LOTS_PER_BASE_UNIT
    pub(crate) fn compute_fees_after_matching_concludes(&mut self) {
        let fee_in_adjusted_quote_lots = compute_fee_v2(self.matched_adjusted_quote_lots);

        // Fee rounded up to a multiple of BASE_LOTS_PER_BASE_UNIT
        let rounded_fee_in_adjusted_quote_lots =
            round_adjusted_quote_lots_up(fee_in_adjusted_quote_lots);

        // fee_in_adjusted_quote_lots
        self.quote_lot_fees = rounded_fee_in_adjusted_quote_lots / BASE_LOTS_PER_BASE_UNIT;
    }

    pub(crate) fn compute_fees_after_matching_concludes_v2(&mut self) {
        let fee_in_adjusted_quote_lots = compute_fee_v2(self.matched_adjusted_quote_lots);

        // fee_in_adjusted_quote_lots
        self.quote_lot_fees =
            compute_quote_lots_from_adjusted_quote_lots_ceil(fee_in_adjusted_quote_lots);
    }

    // compute_fees() always rounds up, but matched_quote_lots() rounds down for asks.
    // Can we still do addition here?
    pub(crate) fn matched_quote_lots_v2(&self) -> QuoteLots {
        // TODO quote_lot_fees used anywhere else? We could compute matched_quote_lots()
        // directly as round_adjusted_quote_lots_up(self.matched_adjusted_quote_lots +- compute_fee(self.matched_adjusted_quote_lots)) / BASE_LOTS_PER_BASE_UNIT)
        match self.side {
            // We add the quote_lot_fees to account for the fee being paid on a buy order
            Side::Bid => {
                compute_quote_lots_from_adjusted_quote_lots_ceil(self.matched_adjusted_quote_lots)
                    + self.quote_lot_fees
            }
            // We subtract the quote_lot_fees to account for the fee being paid on a sell order
            Side::Ask => {
                // Why is lhs rounded down but rhs is rounded up?
                compute_quote_lots_from_adjusted_quote_lots_floor(self.matched_adjusted_quote_lots)
                    - self.quote_lot_fees
            }
        }
    }

    /// Computes matched quote lots from matched_adjusted_quote_lots after matching
    /// is complete.
    ///
    /// `matched_adjusted_quote_lots` is rounded down to the nearest tick
    /// for buys and up for sells to yield a whole number of matched_quote_lots.
    pub(crate) fn matched_quote_lots(&self) -> QuoteLots {
        // TODO quote_lot_fees used anywhere else? We could compute matched_quote_lots()
        // directly as round_adjusted_quote_lots_up(self.matched_adjusted_quote_lots +- compute_fee(self.matched_adjusted_quote_lots)) / BASE_LOTS_PER_BASE_UNIT)
        match self.side {
            // We add the quote_lot_fees to account for the fee being paid on a buy order
            Side::Bid => {
                (round_adjusted_quote_lots_up(self.matched_adjusted_quote_lots)
                    / BASE_LOTS_PER_BASE_UNIT)
                    + self.quote_lot_fees
            }
            // We subtract the quote_lot_fees to account for the fee being paid on a sell order
            Side::Ask => {
                (round_adjusted_quote_lots_down(self.matched_adjusted_quote_lots)
                    / BASE_LOTS_PER_BASE_UNIT)
                    - self.quote_lot_fees
            }
        }
    }

    /// Number of base lots available to match at a given price
    ///
    /// # Arguments
    ///
    /// * `price_in_ticks` - Price where the inflight order is getting matched
    pub fn base_lots_available_to_match(&self, price_in_ticks: Ticks) -> BaseLots {
        self.base_lot_budget.min(
            self.adjusted_quote_lot_budget
                .unchecked_div::<QuoteLotsPerBaseUnit, BaseLots>(
                    price_in_ticks * TICK_SIZE_IN_QUOTE_LOTS_PER_BASE_UNIT,
                ),
        )
    }
}
