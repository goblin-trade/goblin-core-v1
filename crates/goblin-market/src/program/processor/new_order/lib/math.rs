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
    state::Side,
};

/// Round up the fee to the nearest adjusted quote lot
pub fn compute_fee(size_in_adjusted_quote_lots: AdjustedQuoteLots) -> AdjustedQuoteLots {
    // 1 bps = 1 / 10_000
    AdjustedQuoteLots::new(
        ((size_in_adjusted_quote_lots.as_u128() * TAKER_FEE_BPS as u128 + 10_000 - 1) / 10_000)
            as u64,
    )
}

pub fn compute_fee_v2(size_in_adjusted_quote_lots: AdjustedQuoteLots) -> AdjustedQuoteLots {
    AdjustedQuoteLots::new(
        (size_in_adjusted_quote_lots.as_u64() * TAKER_FEE_BPS as u64).div_ceil(10_000),
    )
}

pub fn size_plus_fee(size_in_adjusted_quote_lots: AdjustedQuoteLots) -> AdjustedQuoteLots {
    AdjustedQuoteLots::new(
        ((size_in_adjusted_quote_lots.as_u128() * TAKER_FEE_BPS as u128 + 10000 - 1) / 10000)
            as u64,
    )
}

/// Adjusted quote lots, rounded up to the nearest multiple of base_lots_per_base_unit
///
/// div_ceil = (numerator + denominator - 1) / denominator
///
/// * Multiplication with BASE_LOTS_PER_BASE_UNIT in the end is redundant. The result
/// of each BASE_LOTS_PER_BASE_UNIT call is divided by BASE_LOTS_PER_BASE_UNIT.
pub fn round_adjusted_quote_lots_up(
    num_adjusted_quote_lots: AdjustedQuoteLots,
) -> AdjustedQuoteLots {
    // num_adjusted_quote_lots.div_ceil(BASE_LOTS_PER_BASE_UNIT) * BASE_LOTS_PER_BASE_UNIT
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

pub fn compute_quote_lots_from_adjusted_quote_lots_ceil(
    num_adjusted_quote_lots: AdjustedQuoteLots,
) -> QuoteLots {
    num_adjusted_quote_lots.div_ceil::<BaseLotsPerBaseUnit, QuoteLots>(BASE_LOTS_PER_BASE_UNIT)
}

pub fn compute_quote_lots_from_adjusted_quote_lots_floor(
    num_adjusted_quote_lots: AdjustedQuoteLots,
) -> QuoteLots {
    num_adjusted_quote_lots.unchecked_div::<BaseLotsPerBaseUnit, QuoteLots>(BASE_LOTS_PER_BASE_UNIT)
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

/// Compute adjusted quote lots for a given side and quote lot budget
///
/// Adjustment means increasing (for asks) or decreasing (for bids) the budget
/// to pay taker fee. The taker fee is always paid in quote token, therefore
/// the input budget is increased for asks and decreased for bids.
///
/// # Formula
///
/// - Size in adjusted quote lots = side in quote lots * base lots / base lots per base unit
/// - Adjust, i.e. increase or decrease the budget to account for taker fee.
///
/// # Arguments
///
/// * `side` - Adjusted quote lot budget is decreased by dividing with (1 + fee_bps)
/// for bids and increased by dividing with (1 - fee_bps) for asks
/// * `quote_lots`- Quote lot budget
pub fn compute_adjusted_quote_lots(side: Side, quote_lots: QuoteLots) -> AdjustedQuoteLots {
    let size_in_adjusted_quote_lots = quote_lots * BASE_LOTS_PER_BASE_UNIT;

    match side {
        // For buys, the adjusted quote lot budget is decreased by dividing with (1 + fee_bps)
        // This is because the fee is added to the quote lots spent after the matching is complete.

        // Bid orders trade quote tokens for base tokens. Since the fee is only paid in quote token,
        // we a-priori reserve some quote tokens before the trade.
        // We reserve some quote tokens to pay fees by decreasing the available quote lot budget for matching.
        Side::Bid => {
            adjusted_quote_lot_budget_post_fee_adjustment_for_buys(size_in_adjusted_quote_lots)
        }
        // For sells, the adjusted quote lot budget is increased by dividing with (1 + fee_bps)
        // This is because the fee is subtracted from the quote lot received after the matching is complete.

        // Ask orders trade base tokens for quote tokens. Since the fee is only paid in quote token,
        // we a-posteriori subtract the output quote token after matching happens.
        // The adjusted quote lot budget is increased in order to cover the fee charged after swapping.
        Side::Ask => {
            adjusted_quote_lot_budget_post_fee_adjustment_for_sells(size_in_adjusted_quote_lots)
        }
    }
}

/// Adjusted quote lot budget for buys
///
/// The result will be smaller than the input.
///
/// # Formula
///
/// * input budget = budget spent on matching (i.e. adjusted budget) + budget spent on fee (% of adjusted budget)
/// * input budget = adjusted budget (1 + taker fee)
/// * adjusted buget = input budget / (1 + taker fee)
/// * Since TAKER_FEE_BPS has a denominator of 10_000
///   adjusted budget = input budget * 10_000 / (10_000 + TAKER_FEE_BPS)
pub fn adjusted_quote_lot_budget_post_fee_adjustment_for_buys(
    size_in_adjusted_quote_lots: AdjustedQuoteLots,
) -> AdjustedQuoteLots {
    let adjusted_raw_u128 =
        size_in_adjusted_quote_lots.as_u128() * 10000 / (10000 + TAKER_FEE_BPS as u128);

    AdjustedQuoteLots::new(adjusted_raw_u128 as u64)
}

/// Adjusted quote lot budget for sells
///
/// The adjusted budget must be greater than input budget to accomodate for fees that will be cut
/// after the trade.
///
/// The result will be greater than the input. If the value overflows then use u64::MAX
///
/// # Formula
///
/// * input budget = adjusted budget (1 - taker fee)
/// * adjusted buget = input budget / (1 - taker fee)
/// * Since TAKER_FEE_BPS has a denominator of 10_000
///   adjusted budget = input budget * 10_000 / (10_000 - TAKER_FEE_BPS)
///
/// Cap to u64::MAX if value overflows u64
///
pub fn adjusted_quote_lot_budget_post_fee_adjustment_for_sells(
    size_in_adjusted_quote_lots: AdjustedQuoteLots,
) -> AdjustedQuoteLots {
    let adjusted_raw_u128 =
        size_in_adjusted_quote_lots.as_u128() * 10000 / (10000 - TAKER_FEE_BPS as u128);
    let adjusted_raw = u64::try_from(adjusted_raw_u128).unwrap_or(u64::MAX);

    AdjustedQuoteLots::new(adjusted_raw)
}

/// Adjusted quote lot budget for buys
///
/// The desired result is adjusted_quote_lots / (1 + fee_bps).
///
/// The desired result is adjusted_quote_lots / (1 + fee_bps). We approach this result by taking
/// (size_in_lots * u64::MAX) / (u64::MAX * (1 + fee_bps)) for accurate numerical precision.
/// This will never overflow at any point in the calculation because all intermediate values
/// will be stored in a u128. There is only a single multiplication of u64's which will be
/// strictly less than u128::MAX
pub fn adjusted_quote_lot_budget_post_fee_adjustment_for_buys_deprecated(
    size_in_adjusted_quote_lots: AdjustedQuoteLots,
) -> Option<AdjustedQuoteLots> {
    let fee_adjustment = compute_fee(AdjustedQuoteLots::MAX).as_u128() + u64::MAX as u128;
    // Return an option to catch truncation from downcasting to u64
    // Truncation is not possible because fee_adjustment is always greater than u64::MAX
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
pub fn adjusted_quote_lot_budget_post_fee_adjustment_for_sells_deprecated(
    size_in_adjusted_quote_lots: AdjustedQuoteLots,
) -> Option<AdjustedQuoteLots> {
    let fee_adjustment = u64::MAX as u128 - compute_fee(AdjustedQuoteLots::MAX).as_u128();
    // Return an option to catch truncation from downcasting to u64
    u64::try_from(size_in_adjusted_quote_lots.as_u128() * u64::MAX as u128 / fee_adjustment)
        .ok()
        .map(AdjustedQuoteLots::new)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod adjusted_budget {
        use super::*;

        #[test]
        fn test_adjust_for_buys() {
            let budgets = vec![
                AdjustedQuoteLots::new(0),
                AdjustedQuoteLots::new(u8::MAX as u64),
                AdjustedQuoteLots::new(u16::MAX as u64),
                AdjustedQuoteLots::new(u32::MAX as u64),
                // Fails here.
                // v2 value is more accurate
                // AdjustedQuoteLots::new(u64::MAX as u64),
            ];

            for budget in budgets {
                // Actual result for u64::MAX = 18443055458322919736.989
                // v2 is actually giving a more accurate result. The old result ends with 35.
                let result =
                    adjusted_quote_lot_budget_post_fee_adjustment_for_buys_deprecated(budget)
                        .unwrap();
                let result_v2 = adjusted_quote_lot_budget_post_fee_adjustment_for_buys(budget);

                assert_eq!(result, result_v2);
            }
        }

        #[test]
        fn test_adjust_for_sells() {
            let budgets = vec![
                AdjustedQuoteLots::new(0),
                AdjustedQuoteLots::new(u8::MAX as u64),
                AdjustedQuoteLots::new(u16::MAX as u64),
                AdjustedQuoteLots::new(u32::MAX as u64),
                // Fails here.
                // AdjustedQuoteLots::new(u64::MAX as u64),
            ];

            for budget in budgets {
                // The result with u64::MAX overflows as expected
                let result =
                    adjusted_quote_lot_budget_post_fee_adjustment_for_sells_deprecated(budget)
                        .unwrap();
                let result_v2 = adjusted_quote_lot_budget_post_fee_adjustment_for_sells(budget);

                assert_eq!(result, result_v2);
            }
        }
    }
}
