use crate::quantities::{BaseLots, QuoteLots, WrapperU64};

pub fn process_deposit_funds(base_lots_to_deposit: u64, quote_lots_to_deposit: u64) {
    let quote_lots = QuoteLots::new(quote_lots_to_deposit);
    let base_lots = BaseLots::new(base_lots_to_deposit);
}
