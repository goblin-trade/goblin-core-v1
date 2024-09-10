use stylus_sdk::alloy_primitives::Address;

use crate::{
    program::{
        error::GoblinResult, token_utils::try_withdraw,
        types::matching_engine_response::MatchingEngineResponse,
    },
    quantities::{BaseAtomsRaw, BaseLots, QuoteAtomsRaw, QuoteLots},
    state::{SlotActions, SlotStorage, TraderState},
    GoblinMarket,
};

/// Withdraw free funds for a given trader
///
/// # Arguments
///
/// * `trader` - Withdraw funds from this trader
/// * `recipient` - Credit to this wallet
/// * `num_quote_lots` - Number of lots to withdraw. Pass 0 if none should be withdrawn.
/// Pass U64::MAX to withdraw all.
/// * `num_base_lots` - Number of lots to withdraw. Pass 0 if none should be withdrawn.
/// Pass U64::MAX to withdraw all.
///
pub fn process_withdraw_funds(
    context: &mut GoblinMarket,
    trader: Address,
    recipient: Address,
    quote_lots: QuoteLots,
    base_lots: BaseLots,
) -> GoblinResult<()> {
    let slot_storage = &mut SlotStorage::new();

    // Read
    let mut trader_state = TraderState::read_from_slot(slot_storage, trader);

    // Mutate
    let MatchingEngineResponse {
        num_quote_lots_out,
        num_base_lots_out,
        ..
    } = trader_state.claim_funds_inner(quote_lots, base_lots);

    // Write
    trader_state.write_to_slot(slot_storage, trader);
    SlotStorage::storage_flush_cache(true);

    // Transfer
    let quote_amount_raw = QuoteAtomsRaw::from_lots(num_quote_lots_out);
    let base_amount_raw = BaseAtomsRaw::from_lots(num_base_lots_out);
    try_withdraw(context, quote_amount_raw, base_amount_raw, recipient)?;

    // There is no eviction

    Ok(())
}
