use stylus_sdk::alloy_primitives::Address;

use crate::{
    error::GoblinResult,
    quantities::{BaseLots, QuoteLots, WrapperU64},
    state::{FIFOMarket, SlotActions, SlotStorage},
    GoblinMarket,
};

pub fn process_withdraw_funds(
    context: &mut GoblinMarket,
    trader: Address,
    base_lots_to_withdraw: u64,
    quote_lots_to_withdraw: u64,
) -> GoblinResult<()> {
    let quote_lots = QuoteLots::new(quote_lots_to_withdraw);
    let base_lots = BaseLots::new(base_lots_to_withdraw);

    // Load market
    let mut slot_storage = SlotStorage::new();
    let market = FIFOMarket::read_from_slot(&slot_storage);

    Ok(())
}

// There is no eviction
