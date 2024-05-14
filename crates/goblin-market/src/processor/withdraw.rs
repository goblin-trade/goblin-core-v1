use stylus_sdk::alloy_primitives::Address;

use crate::{error::GoblinResult, GoblinMarket};

pub fn process_withdraw_funds(
    context: &mut GoblinMarket,
    trader: Address,
    base_lots_to_withdraw: u64,
    quote_lots_to_withdraw: u64,
) -> GoblinResult<()> {
    // Load market

    Ok(())
}

// There is no eviction
