/// Fee calculation utilities
///
/// Facts about fees
///
/// * Fees are charged in the taker token
///
/// * Fees are only charged on take orders. Post-only and limit orders are free.
///
/// * For bids: Quote lots are exchanged for base lots. Budget is decreased by the max fee because
/// fees are added to the quote lots spent after matching is complete.
///
/// * For asks: Base lots are exchanged for quote lots. Budget is increased by max fee because
/// fees will be subtracted after matching is complete.
///
use crate::{
    parameters::TAKER_FEE_BPS,
    quantities::{AdjustedQuoteLots, WrapperU64},
};

/// Round up the fee to the nearest adjusted quote lot
pub fn compute_fee(size_in_adjusted_quote_lots: AdjustedQuoteLots) -> AdjustedQuoteLots {
    AdjustedQuoteLots::new(
        ((size_in_adjusted_quote_lots.as_u128() * TAKER_FEE_BPS as u128 + 10000 - 1) / 10000)
            as u64,
    )
}

/// Quote lot budget with fees adjusted (buys)
///
/// The desired result is adjusted_quote_lots / (1 + fee_bps). We approach this result by taking
/// (size_in_lots * u64::MAX) / (u64::MAX * (1 + fee_bps)) for accurate numerical precision.
/// This will never overflow at any point in the calculation because all intermediate values
/// will be stored in a u128. There is only a single multiplication of u64's which will be
/// strictly less than u128::MAX
pub fn adjusted_quote_lot_budget_post_fee_adjustment_for_buys(
    size_in_adjusted_quote_lots: AdjustedQuoteLots,
) -> Option<AdjustedQuoteLots> {
    let fee_adjustment = compute_fee(AdjustedQuoteLots::MAX).as_u128() + u64::MAX as u128;
    // Return an option to catch truncation from downcasting to u64
    u64::try_from(size_in_adjusted_quote_lots.as_u128() * u64::MAX as u128 / fee_adjustment)
        .ok()
        .map(AdjustedQuoteLots::new)
}

/// Quote lot budget with fees adjusted (sells)
///
/// The desired result is adjusted_quote_lots / (1 - fee_bps). We approach this result by taking
/// (size_in_lots * u64::MAX) / (u64::MAX * (1 - fee_bps)) for accurate numerical precision.
/// This will never overflow at any point in the calculation because all intermediate values
/// will be stored in a u128. There is only a single multiplication of u64's which will be
/// strictly less than u128::MAX
pub fn adjusted_quote_lot_budget_post_fee_adjustment_for_sells(
    size_in_adjusted_quote_lots: AdjustedQuoteLots,
) -> Option<AdjustedQuoteLots> {
    let fee_adjustment = u64::MAX as u128 - compute_fee(AdjustedQuoteLots::MAX).as_u128();
    // Return an option to catch truncation from downcasting to u64
    u64::try_from(size_in_adjusted_quote_lots.as_u128() * u64::MAX as u128 / fee_adjustment)
        .ok()
        .map(AdjustedQuoteLots::new)
}
