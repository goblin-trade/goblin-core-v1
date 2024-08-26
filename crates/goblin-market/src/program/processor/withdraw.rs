use stylus_sdk::msg;

use crate::{
    program::{error::GoblinResult, token_utils::try_withdraw},
    quantities::{BaseAtomsRaw, BaseLots, QuoteAtomsRaw, QuoteLots, WrapperU64},
    state::{MatchingEngineResponse, SlotActions, SlotStorage},
    GoblinMarket,
};

use crate::state::matching_engine;

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
    let trader = msg::sender();

    let quote_lots = QuoteLots::new(quote_lots_to_withdraw);
    let base_lots = BaseLots::new(base_lots_to_withdraw);

    let slot_storage = &mut SlotStorage::new();

    let MatchingEngineResponse {
        num_quote_lots_out,
        num_base_lots_out,
        ..
    } = matching_engine::claim_funds(slot_storage, trader, quote_lots, base_lots);

    // Transfer
    let quote_amount_raw = QuoteAtomsRaw::from_lots(num_quote_lots_out);
    let base_amount_raw = BaseAtomsRaw::from_lots(num_base_lots_out);

    try_withdraw(context, quote_amount_raw, base_amount_raw, trader)?;

    Ok(())
}

// There is no eviction
