use crate::quantities::{BaseLots, QuoteLots};

/// Represents the change in lots after a matching engine operation
/// from the "trader's perspective"
#[repr(C)]
#[derive(Debug, Eq, PartialEq, Default, Copy, Clone)]
pub struct MatchingEngineResponse {
    /// The number of quote lots received by the matching engine.
    pub num_quote_lots_in: QuoteLots,

    /// The number of base lots received by the matching engine.
    pub num_base_lots_in: BaseLots,

    /// The number of quote lots emitted by the matching engine.
    pub num_quote_lots_out: QuoteLots,

    /// The number of base lots received by the matching engine.
    pub num_base_lots_out: BaseLots,

    pub num_quote_lots_posted: QuoteLots,
    pub num_base_lots_posted: BaseLots,

    pub num_free_quote_lots_used: QuoteLots,
    pub num_free_base_lots_used: BaseLots,
}

impl MatchingEngineResponse {
    /// Constructs a new response for a buy operation. From the trader's perspective,
    /// quote tokens are exchanged for base tokens in a buy operation.
    ///
    /// # Parameters
    /// - `num_quote_lots_in`: The number of quote lots received.
    /// - `num_base_lots_out`: The number of base lots sold.
    ///
    /// # Returns
    /// A `MatchingEngineResponse` initialized for a buy operation.
    pub fn new_from_buy(num_quote_lots_in: QuoteLots, num_base_lots_out: BaseLots) -> Self {
        MatchingEngineResponse {
            num_quote_lots_in,
            num_base_lots_in: BaseLots::ZERO,
            num_quote_lots_out: QuoteLots::ZERO,
            num_base_lots_out,
            num_quote_lots_posted: QuoteLots::ZERO,
            num_base_lots_posted: BaseLots::ZERO,
            num_free_quote_lots_used: QuoteLots::ZERO,
            num_free_base_lots_used: BaseLots::ZERO,
        }
    }

    /// Constructs a new response for a sell operation. From the trader's perspective
    /// base tokens are exchanged for quote tokens in a sell operation.
    ///
    /// # Parameters
    /// - `num_base_lots_in`: The number of base lots received.
    /// - `num_quote_lots_out`: The number of quote lots sold.
    ///
    /// # Returns
    /// A `MatchingEngineResponse` initialized for a sell operation.
    pub fn new_from_sell(num_base_lots_in: BaseLots, num_quote_lots_out: QuoteLots) -> Self {
        MatchingEngineResponse {
            num_quote_lots_in: QuoteLots::ZERO,
            num_base_lots_in,
            num_quote_lots_out,
            num_base_lots_out: BaseLots::ZERO,
            num_quote_lots_posted: QuoteLots::ZERO,
            num_base_lots_posted: BaseLots::ZERO,
            num_free_quote_lots_used: QuoteLots::ZERO,
            num_free_base_lots_used: BaseLots::ZERO,
        }
    }

    /// Constructs a new response for a withdrawal operation.
    ///
    /// # Parameters
    /// - `num_base_lots_out`: The number of base lots withdrawn.
    /// - `num_quote_lots_out`: The number of quote lots withdrawn.
    ///
    /// # Returns
    /// A `MatchingEngineResponse` initialized for a withdrawal operation.
    pub fn new_withdraw(num_base_lots_out: BaseLots, num_quote_lots_out: QuoteLots) -> Self {
        MatchingEngineResponse {
            num_quote_lots_in: QuoteLots::ZERO,
            num_base_lots_in: BaseLots::ZERO,
            num_quote_lots_out,
            num_base_lots_out,
            num_quote_lots_posted: QuoteLots::ZERO,
            num_base_lots_posted: BaseLots::ZERO,
            num_free_quote_lots_used: QuoteLots::ZERO,
            num_free_base_lots_used: BaseLots::ZERO,
        }
    }

    /// Adds the specified number of quote lots to the posted quote lots.
    ///
    /// # Parameters
    /// - `num_quote_lots`: The number of quote lots to post.
    #[inline(always)]
    pub fn post_quote_lots(&mut self, num_quote_lots: QuoteLots) {
        self.num_quote_lots_posted += num_quote_lots;
    }

    /// Adds the specified number of base lots to the posted base lots.
    ///
    /// # Parameters
    /// - `num_base_lots`: The number of base lots to post.
    #[inline(always)]
    pub fn post_base_lots(&mut self, num_base_lots: BaseLots) {
        self.num_base_lots_posted += num_base_lots;
    }

    /// Calculates the total number of base lots involved in the operation
    /// (both incoming and outgoing).
    ///
    /// # Returns
    /// The total number of base lots.
    #[inline(always)]
    pub fn num_base_lots(&self) -> BaseLots {
        self.num_base_lots_in + self.num_base_lots_out
    }

    /// Calculates the total number of quote lots involved in the operation
    /// (both incoming and outgoing).
    ///
    /// # Returns
    /// The total number of quote lots.
    #[inline(always)]
    pub fn num_quote_lots(&self) -> QuoteLots {
        self.num_quote_lots_in + self.num_quote_lots_out
    }

    #[inline(always)]
    pub fn use_free_quote_lots(&mut self, num_quote_lots: QuoteLots) {
        self.num_free_quote_lots_used += num_quote_lots;
    }

    #[inline(always)]
    pub fn use_free_base_lots(&mut self, num_base_lots: BaseLots) {
        self.num_free_base_lots_used += num_base_lots;
    }

    /// Calculates the total deposit amount in quote lots for bids.
    ///
    /// # Returns
    /// The total number of quote lots deposited, adjusted by the posted and free lots.
    #[inline(always)]
    pub fn get_deposit_amount_bid_in_quote_lots(&self) -> QuoteLots {
        self.num_quote_lots_in + self.num_quote_lots_posted - self.num_free_quote_lots_used
    }

    #[inline(always)]
    pub fn get_deposit_amount_ask_in_base_lots(&self) -> BaseLots {
        self.num_base_lots_in + self.num_base_lots_posted - self.num_free_base_lots_used
    }

    #[inline(always)]
    pub fn verify_no_deposit(&self) -> bool {
        self.num_base_lots_in + self.num_base_lots_posted == self.num_free_base_lots_used
            && self.num_quote_lots_in + self.num_quote_lots_posted == self.num_free_quote_lots_used
    }

    #[inline(always)]
    pub fn verify_no_withdrawal(&self) -> bool {
        self.num_base_lots_out == BaseLots::ZERO && self.num_quote_lots_out == QuoteLots::ZERO
    }
}
