use slot_storage::{SlotActions, SlotStorage};
use stylus_sdk::alloy_primitives::Address;

use crate::{
    error::GoblinResult,
    parameters::{BASE_LOT_SIZE, QUOTE_LOT_SIZE},
    quantities::{get_base_atoms_raw, get_quote_atoms_raw, BaseLots, QuoteLots, WrapperU64},
    state::{slot_storage, FIFOMarket, Market},
    token_utils::try_deposit,
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
    let quote_amount = quote_lots * QUOTE_LOT_SIZE;
    let base_amount = base_lots * BASE_LOT_SIZE;

    let quote_amount_raw = get_quote_atoms_raw(quote_amount);
    let base_amount_raw = get_base_atoms_raw(base_amount);

    try_deposit(context, base_amount_raw, quote_amount_raw, trader)?;

    Ok(())
}
