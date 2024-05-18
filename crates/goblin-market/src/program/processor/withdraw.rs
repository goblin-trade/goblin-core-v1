use stylus_sdk::msg;

use crate::{
    parameters::{BASE_LOT_SIZE, QUOTE_LOT_SIZE},
    program::error::{GoblinError, GoblinResult, WithdrawFundsError},
    program::token_utils::try_withdraw,
    quantities::{get_base_atoms_raw, get_quote_atoms_raw, BaseLots, QuoteLots, WrapperU64},
    state::{FIFOMarket, MatchingEngineResponse, SlotActions, SlotStorage, WritableMarket},
    GoblinMarket,
};

/// Withdraw from free funds
///
/// # Arguments
///
/// * `quote_lots_to_withdraw` - Quote lots to withdraw. Pass u64::MAX to withdraw all
/// * `base_lots_to_withdraw` - Base lots to withdraw. Pass u32::MAX to withdraw all
///
/// TODO check- quote amount for deposits could be u64. Just make sure that max amount in
/// resting orders is u32
///
pub fn process_withdraw_funds(
    context: &mut GoblinMarket,
    quote_lots_to_withdraw: u64,
    base_lots_to_withdraw: u64,
) -> GoblinResult<()> {
    let quote_lots = QuoteLots::new(quote_lots_to_withdraw);
    let base_lots = BaseLots::new(base_lots_to_withdraw);

    let trader = msg::sender();

    // Load market
    let mut slot_storage = SlotStorage::new();
    let market = FIFOMarket::read_from_slot(&slot_storage);

    let MatchingEngineResponse {
        num_quote_lots_out,
        num_base_lots_out,
        ..
    } = market
        .claim_funds(&mut slot_storage, trader, quote_lots, base_lots)
        .ok_or(GoblinError::WithdrawFundsError(WithdrawFundsError {}))?;

    SlotStorage::storage_flush_cache(true);

    let quote_amount = num_quote_lots_out * QUOTE_LOT_SIZE;
    let base_amount = num_base_lots_out * BASE_LOT_SIZE;

    let quote_amount_raw = get_quote_atoms_raw(quote_amount);
    let base_amount_raw = get_base_atoms_raw(base_amount);

    try_withdraw(context, quote_amount_raw, base_amount_raw, trader)?;

    Ok(())
}

// There is no eviction
