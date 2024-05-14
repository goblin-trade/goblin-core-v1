use stylus_sdk::alloy_primitives::Address;

use crate::{
    error::GoblinResult,
    quantities::{BaseLots, QuoteLots, WrapperU64},
};

pub fn process_deposit_funds(
    trader: Address,
    base_lots_to_deposit: u64,
    quote_lots_to_deposit: u64,
) -> GoblinResult<()> {
    let quote_lots = QuoteLots::new(quote_lots_to_deposit);
    let base_lots = BaseLots::new(base_lots_to_deposit);

    Ok(())
}
