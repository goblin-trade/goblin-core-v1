use crate::quantities::{BaseLots, QuoteLots};

/// Represents the change in lots after a matching engine operation
/// from the "trader's perspective"
#[repr(C)]
#[derive(Debug, Eq, PartialEq, Default, Copy, Clone)]
pub struct MatchingEngineResponse {
    /// The number of quote lots to be transferred in by the trader to the matching engine
    /// after an IOC or limit 'Bid / Buy' is matched.
    pub num_quote_lots_in: QuoteLots,

    /// The number of quote lots to be transferred in by the trader to the matching engine
    /// after an IOC or limit 'Ask / Sell' is matched.
    pub num_base_lots_in: BaseLots,

    /// The number of quote lots to be transferred out by the matching engine to the trader
    /// after an IOC or limit 'Ask / Sell' is matched, or when free tokens are withdrawn
    /// from the trader state.
    pub num_quote_lots_out: QuoteLots,

    /// The number of base lots to be transferred out by the matching engine to the trader
    /// after an IOC or limit 'Bid / Buy' is matched, or when free tokens are withdrawn
    /// from the trader state.
    pub num_base_lots_out: BaseLots,

    /// The number of quote lots 'posted' to post a Bid / Buy limit order on the book.
    /// An equal number of lots are locked up in the trader state.
    pub num_quote_lots_posted: QuoteLots,

    /// The number of base lots 'posted' to post an Ask / Sell limit order on the book.
    /// An equal number of lots are locked up in the trader state.
    pub num_base_lots_posted: BaseLots,

    /// The number of free quote lots used up from trader state.
    pub num_free_quote_lots_used: QuoteLots,

    /// The number of free base lots used up from trader state.
    pub num_free_base_lots_used: BaseLots,
}

impl MatchingEngineResponse {
    /// Constructs a new response for an executed taker 'Bid / Buy' operation. It is generated
    /// when an IOC or limit order is matched.
    ///
    /// Base lots (output) are bought with quote lots (input).
    ///
    /// # Parameters
    /// - `num_quote_lots_in`: The number of quote lots paid by the trader.
    /// - `num_base_lots_out`: The number of base lots bought by trader.
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

    /// Constructs a new response for an executed taker 'Ask / Sell' operation. It is generated
    /// when an IOC or limit order is matched.
    ///
    /// Base lots (input) are sold for quote lots (output).
    ///
    /// # Parameters
    /// - `num_base_lots_in`: The number of base lots sold by the trader.
    /// - `num_quote_lots_out`: The number of quote lots received by the trader.
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

    /// Post, i.e. lock up quote lots to 'post' a 'Bid / Buy' limit order on the book.
    /// A corresponding number of lots are locked up in the trader state.
    ///
    /// This function is only called for post-only and limit orders, not IOC orders.
    ///
    /// # Parameters
    /// - `num_quote_lots`: The number of quote lots to post on the book.
    #[inline(always)]
    pub fn post_quote_lots(&mut self, num_quote_lots: QuoteLots) {
        self.num_quote_lots_posted += num_quote_lots;
    }

    /// Post, i.e. lock up base lots to 'post' an 'Ask / Sell' limit order on the book.
    /// A corresponding number of lots are locked up in the trader state.
    ///
    /// This function is only called for post-only and limit orders, not IOC orders.
    ///
    /// # Parameters
    /// - `num_quote_lots`: The number of base lots to post on the book.
    #[inline(always)]
    pub fn post_base_lots(&mut self, num_base_lots: BaseLots) {
        self.num_base_lots_posted += num_base_lots;
    }

    /// Calculates the total number of base lots involved in the operation
    /// (both incoming and outgoing).
    ///
    /// Either `num_base_lots_in` or `num_base_lots_out` is guaranteed to be zero.
    /// We add the two to avoid using if-else for `side`.
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

    /// Called when free quote lots are deducted from trader state. Tracks the total number
    /// of free quote lots used up
    ///
    /// # Arguments
    ///
    /// * `num_quote_lots`
    #[inline(always)]
    pub fn use_free_quote_lots(&mut self, num_quote_lots: QuoteLots) {
        self.num_free_quote_lots_used += num_quote_lots;
    }

    /// Called when free base lots are deducted from trader state. Tracks the total number
    /// of free base lots used up
    ///
    /// # Arguments
    ///
    /// * `num_base_lots`
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

    /// Whether minimum lot requirements are met for an IOC order
    ///
    /// # Arguments
    ///
    /// * `base_lots` - Minimum base lots to fill
    /// * `quote_lots` - Minimum quote lots to fill
    #[inline(always)]
    pub fn verify_minimum_lots_filled(&self, base_lots: BaseLots, quote_lots: QuoteLots) -> bool {
        self.num_base_lots() >= base_lots && self.num_quote_lots() >= quote_lots
    }
}
