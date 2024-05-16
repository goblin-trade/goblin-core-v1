use stylus_sdk::alloy_primitives::Address;

use crate::program::checkers::assert_valid_fee_collector;
use crate::{
    parameters::{QUOTE_LOT_SIZE, QUOTE_TOKEN},
    program::{maybe_invoke_withdraw, GoblinResult},
    quantities::get_quote_atoms_raw,
    state::{FIFOMarket, SlotActions, SlotStorage, WritableMarket},
    GoblinMarket,
};

/// Collect protocol fees
///
/// msg::sender() should be the FEE_COLLECTOR
///
/// # Parameters
///
/// * `recipient` - Transfer fees to this address
///
pub fn process_collect_fees(context: &mut GoblinMarket, recipient: Address) -> GoblinResult<()> {
    assert_valid_fee_collector()?;

    let mut slot_storage = SlotStorage::new();
    let mut market = FIFOMarket::read_from_slot(&slot_storage);

    let num_quote_lots_out = market.collect_fees(&mut slot_storage);

    // write market to slot
    SlotStorage::storage_flush_cache(true);

    let quote_atoms_collected = num_quote_lots_out * QUOTE_LOT_SIZE;
    let quote_atoms_collected_raw = get_quote_atoms_raw(quote_atoms_collected);

    maybe_invoke_withdraw(context, quote_atoms_collected_raw, QUOTE_TOKEN, recipient)?;

    Ok(())
}
