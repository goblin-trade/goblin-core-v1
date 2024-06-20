use alloc::vec::Vec;
use stylus_sdk::alloy_primitives::{Address, B256};

use crate::{
    program::{GoblinError, GoblinResult, UndefinedFailedMultipleLimitOrderBehavior},
    quantities::{BaseLots, Ticks, WrapperU64},
    state::{MatchingEngine, SlotActions, SlotStorage},
};

pub enum FailedMultipleLimitOrderBehavior {
    /// Orders will never cross the spread. Instead they will be amended to the closest non-crossing price.
    /// The entire transaction will fail if matching engine returns None for any order, which indicates an error.
    ///
    /// If an order has insufficient funds, the entire transaction will fail.
    FailOnInsufficientFundsAndAmendOnCross = 0,

    /// If any order crosses the spread or has insufficient funds, the entire transaction will fail.
    FailOnInsufficientFundsAndFailOnCross = 1,

    /// Orders will be skipped if the user has insufficient funds.
    /// Crossing orders will be amended to the closest non-crossing price.
    SkipOnInsufficientFundsAndAmendOnCross = 2,

    /// Orders will be skipped if the user has insufficient funds.
    /// If any order crosses the spread, the entire transaction will fail.
    SkipOnInsufficientFundsAndFailOnCross = 3,
}

impl FailedMultipleLimitOrderBehavior {
    pub fn decode(value: u8) -> GoblinResult<FailedMultipleLimitOrderBehavior> {
        match value {
            0 => Ok(FailedMultipleLimitOrderBehavior::FailOnInsufficientFundsAndAmendOnCross),
            1 => Ok(FailedMultipleLimitOrderBehavior::FailOnInsufficientFundsAndFailOnCross),
            2 => Ok(FailedMultipleLimitOrderBehavior::SkipOnInsufficientFundsAndAmendOnCross),
            3 => Ok(FailedMultipleLimitOrderBehavior::SkipOnInsufficientFundsAndFailOnCross),
            _ => Err(GoblinError::UndefinedFailedMultipleLimitOrderBehavior(
                UndefinedFailedMultipleLimitOrderBehavior {},
            )),
        }
    }

    pub fn should_fail_on_cross(&self) -> bool {
        matches!(
            self,
            FailedMultipleLimitOrderBehavior::FailOnInsufficientFundsAndFailOnCross
                | FailedMultipleLimitOrderBehavior::SkipOnInsufficientFundsAndFailOnCross
        )
    }

    pub fn should_skip_orders_with_insufficient_funds(&self) -> bool {
        matches!(
            self,
            FailedMultipleLimitOrderBehavior::SkipOnInsufficientFundsAndAmendOnCross
                | FailedMultipleLimitOrderBehavior::SkipOnInsufficientFundsAndFailOnCross
        )
    }
}

pub struct CondensedOrder {
    pub price_in_ticks: Ticks,
    pub size_in_base_lots: BaseLots,
    pub track_block: bool,
    pub last_valid_block_or_unix_timestamp_in_seconds: u32,
}

impl CondensedOrder {
    pub fn decode(bytes: &[u8; 32]) -> Self {
        CondensedOrder {
            price_in_ticks: Ticks::new(u64::from_be_bytes(bytes[0..8].try_into().unwrap())),
            size_in_base_lots: BaseLots::new(u64::from_be_bytes(bytes[8..16].try_into().unwrap())),
            track_block: (bytes[16] & 0b0000_0001) != 0,
            last_valid_block_or_unix_timestamp_in_seconds: u32::from_be_bytes(
                bytes[17..21].try_into().unwrap(),
            ),
        }
    }
}

/// Create multiple new orders
///
/// Each order request is (price in ticks, size in base lots, side)
/// The order ID is derived by reading index list and bitmaps.
/// Note- Side must be known. We don't want an order intended as a bid being placed as an ask.
///
/// Increase feature- placing order at the same price increases the order
/// But this will require us to read the RestingOrder slots one by one to
/// know the best order belonging to the trader. AVOID
/// Alternative- Cancel and place. If done atomically it will have the same or better index
pub fn process_multiple_new_orders(
    to: Address,
    failed_multiple_limit_order_behavior: FailedMultipleLimitOrderBehavior,
    bids: Vec<B256>,
    asks: Vec<B256>,
    client_order_id: u128,
    use_free_funds: bool,
) -> GoblinResult<()> {
    let mut matching_engine = MatchingEngine {
        slot_storage: &mut SlotStorage::new(),
    };

    Ok(())
}
