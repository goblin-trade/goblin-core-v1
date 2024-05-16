use stylus_sdk::alloy_primitives::Address;

use crate::{
    error::GoblinResult,
    parameters::{QUOTE_LOT_SIZE, QUOTE_TOKEN},
    quantities::get_quote_atoms_raw,
    state::{FIFOMarket, SlotActions, SlotStorage, WritableMarket},
    token_utils::maybe_invoke_withdraw,
    validation::checkers::assert_valid_fee_collector,
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

    let num_quote_lots_out = market.collect_fees();

    // write market to slot
    market.write_to_slot(&mut slot_storage);

    let quote_atoms_collected = num_quote_lots_out * QUOTE_LOT_SIZE;
    let quote_atoms_collected_raw = get_quote_atoms_raw(quote_atoms_collected);

    maybe_invoke_withdraw(context, quote_atoms_collected_raw, QUOTE_TOKEN, recipient)?;

    Ok(())
}
