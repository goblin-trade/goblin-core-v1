use stylus_sdk::alloy_primitives::Address;

use crate::program::checkers::assert_valid_fee_collector;
use crate::quantities::QuoteAtomsRaw;
use crate::state::matching_engine;
use crate::{
    parameters::QUOTE_TOKEN,
    program::{maybe_invoke_withdraw, GoblinResult},
    state::{SlotActions, SlotStorage},
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

    let slot_storage = &mut SlotStorage::new();
    let num_quote_lots_out = matching_engine::collect_fees(slot_storage)?;

    // Transfer
    let quote_atoms_collected_raw = QuoteAtomsRaw::from_lots(num_quote_lots_out);

    maybe_invoke_withdraw(
        context,
        quote_atoms_collected_raw.as_u256(),
        QUOTE_TOKEN,
        recipient,
    )?;

    Ok(())
}
