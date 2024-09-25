use context::{ArbContext, ContextActions};
use stylus_sdk::alloy_primitives::Address;

use crate::{
    program::{try_deposit, GoblinResult},
    quantities::{BaseAtomsRaw, BaseLots, QuoteAtomsRaw, QuoteLots},
    state::{context, TraderState},
    GoblinMarket,
};

/// Deposit funds into the trader account. These funds can be used to trade
/// with lower gas costs because ERC20 transfers are avoided.
///
/// A wallet can credit funds to another trader.
///
/// # Arguments
///
/// * `context`
/// * `trader` - Credit funds to this trader. A wallet can credit funds to another trader.
/// * `quote_lots`
/// * `base_lots`
///
pub fn process_deposit_funds(
    context: &mut GoblinMarket,
    trader: Address,
    quote_lots: QuoteLots,
    base_lots: BaseLots,
) -> GoblinResult<()> {
    // Read
    let mut ctx = ArbContext::new();
    let mut trader_state = TraderState::read_from_slot(&ctx, trader);

    // Mutate
    trader_state.deposit_free_base_lots(base_lots);
    trader_state.deposit_free_quote_lots(quote_lots);

    // Write
    trader_state.write_to_slot(&mut ctx, trader);
    ArbContext::storage_flush_cache(true);

    // Transfer
    let quote_amount_raw = QuoteAtomsRaw::from_lots(quote_lots);
    let base_amount_raw = BaseAtomsRaw::from_lots(base_lots);

    try_deposit(context, quote_amount_raw, base_amount_raw, trader)?;

    Ok(())
}
