use crate::quantities::{QuoteLots, Ticks};

/// Holds the best market price and active outer index count for a side
///
/// These two fields are grouped because best market price has meaning only
/// if outer_index_count is on-zero, and these values are updated together.
#[derive(Default, Debug, PartialEq)]
pub struct BestPriceAndIndexCount {
    /// The best market price for the current side, if `outer_index_count` is non-zero
    pub best_price_inner: Ticks,

    /// The number of active outer indices
    pub outer_index_count: u16,
}

impl BestPriceAndIndexCount {
    pub fn best_price(&self) -> Option<Ticks> {
        if self.outer_index_count > 0 {
            Some(self.best_price_inner)
        } else {
            None
        }
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct MarketStateV2 {
    /// Amount of fees collected from the market in its lifetime, in quote lots.
    pub collected_quote_lot_fees: QuoteLots,

    /// Amount of unclaimed fees accrued to the market, in quote lots.
    pub unclaimed_quote_lot_fees: QuoteLots,

    /// Best market price and outer index count for bids
    pub bid_best_price_and_count: BestPriceAndIndexCount,

    /// Best market price and outer index count for asks
    pub ask_best_price_and_count: BestPriceAndIndexCount,
}

impl MarketStateV2 {
    // TODO ensure tick is less than or equal to 2^21 - 1 in post-only and limit
    // order insertions. This way we can call encode() without unwrapping a Result type.
    //
    // New ticks are activated only during insertions. If we enforce max tick
    // bounds during insertion, there is no need to check this here.
    // Matching and removal only removes ticks and moves to the next active tick,
    // it cannot set best price to an arbitrary value.

    // pub fn encode(&self) -> [u8; 32] {
    //     let mut encoded_data = [0u8; 32];

    //     encoded_data[0..8].copy_from_slice(&self.collected_quote_lot_fees.as_u64().to_be_bytes());
    //     encoded_data[8..16].copy_from_slice(&self.unclaimed_quote_lot_fees.as_u64().to_be_bytes());
    //     encoded_data[16..18].copy_from_slice(&self.bids_outer_indices.to_be_bytes());
    //     encoded_data[18..20].copy_from_slice(&self.asks_outer_indices.to_be_bytes());

    //     let best_bid_price = self.best_bid_price.as_u64();
    //     let best_ask_price = self.best_ask_price.as_u64();

    //     // TODO ensure tick is less than or equal to 2^21 - 1 in post-only and limit
    //     // order insertions
    //     //
    //     // New ticks are activated only during insertions. If we enforce max tick
    //     // bounds during insertion, there is no need to check this here.
    //     // Matching and removal only removes ticks and moves to the next active tick,
    //     // it cannot set best price to an arbitrary value.

    //     encoded_data[20..24].copy_from_slice(&(best_bid_price as u32).to_be_bytes());
    //     encoded_data[24..28].copy_from_slice(&(best_ask_price as u32).to_be_bytes());

    //     encoded_data
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_borrow_best_price_with_count() {
        let mut market_state = MarketStateV2::default();

        let bid_best_price_and_count = &mut market_state.bid_best_price_and_count;
        let ask_best_price_and_count = &mut market_state.ask_best_price_and_count;

        bid_best_price_and_count.outer_index_count = 10;
        ask_best_price_and_count.outer_index_count = 1;
    }
}
