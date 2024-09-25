use stylus_sdk::alloy_primitives::Address;

use crate::parameters::FEE_COLLECTOR;
use crate::program::{GoblinError, InvalidFeeCollector};
use crate::quantities::{QuoteAtomsRaw, QuoteLots};
use crate::require;
use crate::state::MarketState;
use crate::{
    parameters::QUOTE_TOKEN,
    program::{maybe_invoke_withdraw, GoblinResult},
    state::{ArbContext, ContextActions},
    GoblinMarket,
};

/// Collect protocol fees
///
/// # Parameters
///
/// * `fee_collector` - Address of the fee collector authority
/// * `recipient` - Transfer fees to this address
///
pub fn process_collect_fees(
    context: &mut GoblinMarket,
    fee_collector: Address,
    recipient: Address,
) -> GoblinResult<()> {
    require!(
        fee_collector == FEE_COLLECTOR,
        GoblinError::InvalidFeeCollector(InvalidFeeCollector {})
    );

    let ctx = &mut ArbContext::new();
    // Read
    let mut market = MarketState::read_from_slot(ctx);
    let num_quote_lots_out = market.unclaimed_quote_lot_fees;

    // Mutate- Mark as claimed
    market.collected_quote_lot_fees += market.unclaimed_quote_lot_fees;
    market.unclaimed_quote_lot_fees = QuoteLots::ZERO;

    // Write
    market.write_to_slot(ctx)?;
    ArbContext::storage_flush_cache(true);

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
