use slot_storage::{SlotActions, SlotStorage};
use stylus_sdk::alloy_primitives::Address;

use crate::{
    program::{try_deposit, GoblinResult},
    quantities::{BaseAtomsRaw, BaseLots, QuoteAtomsRaw, QuoteLots, WrapperU64},
    state::{slot_storage, FIFOMarket, Market},
    GoblinMarket,
};

pub fn process_deposit_funds(
    context: &mut GoblinMarket,
    trader: Address,
    quote_lots_to_deposit: u64,
    base_lots_to_deposit: u64,
) -> GoblinResult<()> {
    let quote_lots = QuoteLots::new(quote_lots_to_deposit);
    let base_lots = BaseLots::new(base_lots_to_deposit);

    let mut slot_storage = SlotStorage::new();
    let mut trader_state = FIFOMarket::get_trader_state(&slot_storage, trader);

    trader_state.deposit_free_base_lots(base_lots);
    trader_state.deposit_free_quote_lots(quote_lots);
    trader_state.write_to_slot(&mut slot_storage, trader);

    // Commit state before making cross-contract call
    SlotStorage::storage_flush_cache(true);

    // Obtain base and quote amounts with resolution
    let quote_amount_raw = QuoteAtomsRaw::from_lots(quote_lots);
    let base_amount_raw = BaseAtomsRaw::from_lots(base_lots);

    try_deposit(context, quote_amount_raw, base_amount_raw, trader)?;

    Ok(())
}
