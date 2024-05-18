use stylus_sdk::alloy_primitives::Address;

use crate::program::checkers::assert_valid_fee_collector;
use crate::quantities::QuoteAtomsRaw;
use crate::{
    parameters::QUOTE_TOKEN,
    program::{maybe_invoke_withdraw, GoblinResult},
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

    let quote_atoms_collected_raw = QuoteAtomsRaw::from_lots(num_quote_lots_out);

    maybe_invoke_withdraw(
        context,
        quote_atoms_collected_raw.as_u256(),
        QUOTE_TOKEN,
        recipient,
    )?;

    Ok(())
}
