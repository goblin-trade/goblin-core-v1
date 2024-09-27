use stylus_sdk::alloy_primitives::Address;

use crate::{
    program::{get_available_base_lots, get_available_quote_lots},
    quantities::{BaseLots, QuoteLots, Ticks},
    state::Side,
    GoblinMarket,
};

use super::compute_quote_lots;

/// A struct to check whether the trader's lots are sufficient for to place an order.
/// It lazy loads the trader's external ERC20 balance if free deposits in trader
/// state fall short.
///
/// TODO test. `GoblinMarket` context needs to be replaced by `ArbContext` first.
pub struct SufficientFundsChecker {
    base_lots_available: BaseLots,
    quote_lots_available: QuoteLots,
    base_allowance_read: bool,
    quote_allowance_read: bool,
}

impl SufficientFundsChecker {
    /// Initialize the cache with balances from trader state.
    /// The balances from token allowances are lazy read.
    ///
    /// # Arguments
    ///
    /// * base_lots_available - Free base lots in trader state
    /// * quote_lots_available - Free quote lots in trader state
    pub fn new(base_lots_available: BaseLots, quote_lots_available: QuoteLots) -> Self {
        SufficientFundsChecker {
            base_lots_available,
            quote_lots_available,
            base_allowance_read: false,
            quote_allowance_read: false,
        }
    }

    /// Whether base lots are sufficient, lazy loading external balance if necessary.
    fn has_sufficient_base_lots(
        &mut self,
        context: &GoblinMarket,
        trader: Address,
        base_lots_required: BaseLots,
    ) -> bool {
        if self.base_lots_available >= base_lots_required {
            return true;
        }

        if self.base_allowance_read {
            return false;
        }

        self.base_lots_available += get_available_base_lots(context, trader);
        self.base_allowance_read = true;
        self.base_lots_available >= base_lots_required
    }

    /// Whether quote lots are sufficient, lazy loading external balance if necessary.
    fn has_sufficient_quote_lots(
        &mut self,
        context: &GoblinMarket,
        trader: Address,
        quote_lots_required: QuoteLots,
    ) -> bool {
        if self.quote_lots_available >= quote_lots_required {
            return true;
        }

        if self.quote_allowance_read {
            return false;
        }

        self.quote_lots_available += get_available_quote_lots(context, trader);
        self.quote_allowance_read = true;
        self.quote_lots_available >= quote_lots_required
    }

    /// Whether a trader has sufficient funds for an order
    pub fn has_sufficient_funds(
        &mut self,
        context: &GoblinMarket,
        trader: Address,
        side: Side,
        price_in_ticks: Ticks,
        num_base_lots: BaseLots,
    ) -> bool {
        match side {
            Side::Bid => {
                let base_lots_required = num_base_lots;
                self.has_sufficient_base_lots(context, trader, base_lots_required)
            }
            Side::Ask => {
                let quote_lots_required = compute_quote_lots(price_in_ticks, num_base_lots);
                self.has_sufficient_quote_lots(context, trader, quote_lots_required)
            }
        }
    }

    pub fn deduct_available_lots(&mut self, base_lots: BaseLots, quote_lots: QuoteLots) {
        debug_assert!(self.base_lots_available >= base_lots);
        debug_assert!(self.quote_lots_available >= quote_lots);

        self.base_lots_available -= base_lots;
        self.quote_lots_available -= quote_lots;
    }
}
