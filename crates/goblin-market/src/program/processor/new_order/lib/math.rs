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
    parameters::{BASE_LOTS_PER_BASE_UNIT, TAKER_FEE_BPS, TICK_SIZE_IN_QUOTE_LOTS_PER_BASE_UNIT},
    quantities::{AdjustedQuoteLots, BaseLots, BaseLotsPerBaseUnit, QuoteLots, Ticks, WrapperU64},
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

/// Adjusted quote lots, rounded up to the nearest multiple of base_lots_per_base_unit
pub fn round_adjusted_quote_lots_up(
    num_adjusted_quote_lots: AdjustedQuoteLots,
) -> AdjustedQuoteLots {
    ((num_adjusted_quote_lots + AdjustedQuoteLots::new(BASE_LOTS_PER_BASE_UNIT.as_u64() - 1))
        .unchecked_div::<BaseLotsPerBaseUnit, QuoteLots>(BASE_LOTS_PER_BASE_UNIT))
        * BASE_LOTS_PER_BASE_UNIT
}

/// Adjusted quote lots, rounded down to the nearest multiple of base_lots_per_base_unit
pub fn round_adjusted_quote_lots_down(
    num_adjusted_quote_lots: AdjustedQuoteLots,
) -> AdjustedQuoteLots {
    num_adjusted_quote_lots.unchecked_div::<BaseLotsPerBaseUnit, QuoteLots>(BASE_LOTS_PER_BASE_UNIT)
        * BASE_LOTS_PER_BASE_UNIT
}

/// Obtain quote lots for an order
///
/// # Formula
///
/// * quote lots Q = PTS / B
/// * Ref- https://ellipsis-labs.gitbook.io/phoenix-dex/tRIkEFlLUzWK9uKO3W2V/getting-started/technical-overview/units#order-sizes
///
/// # Arguments
///
/// * `price_in_ticks`
/// * `base_lots`
///
pub fn compute_quote_lots(price_in_ticks: Ticks, base_lots: BaseLots) -> QuoteLots {
    (price_in_ticks * TICK_SIZE_IN_QUOTE_LOTS_PER_BASE_UNIT * base_lots) / BASE_LOTS_PER_BASE_UNIT
}
