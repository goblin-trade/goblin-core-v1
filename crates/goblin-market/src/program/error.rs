#[cfg(test)]
use core::fmt::Debug;

use alloy_sol_types::sol;
use stylus_sdk::prelude::*;

sol! {
    // Invalid fee collector error
    error InvalidFeeCollector();

    // Exceeded max size of 2^63 - 1 for size of base lots in a resting order
    error ExceedRestingOrderSize();

    // Exceed max tick size of 2^21 - 1
    error ExceedTickSize();

    // Outer indices are not in correct order
    error IndicesNotInOrder();

    // Outer index is not in list
    error IndexNotInList();

    // Failed to reduce error. Thrown in revert_if_fail mode
    error FailedToReduce();

    // Not used yet- to check

    // Invalid instruction data error
    error InvalidInstructionData();

    // Invalid market parameters error
    error InvalidMarketParameters();

    // Market deserialization error
    error FailedToLoadMarketFromAccount();

    // Market already initialized error
    error MarketAlreadyInitialized();

    // Market is not initialized error
    error MarketUninitialized();

    // Invalid state transition error
    error InvalidStateTransition();

    // Invalid market signer error
    error InvalidMarketSigner();

    // Invalid lot size error
    error InvalidLotSize();

    // Invalid tick size error
    error InvalidTickSize();

    // Invalid mint error
    error InvalidMint();

    // Invalid base vault error
    error InvalidBaseVault();

    // Invalid quote vault error
    error InvalidQuoteVault();

    // Invalid base account error
    error InvalidBaseAccount();

    // Invalid quote account error
    error InvalidQuoteAccount();

    // Too many events error
    error TooManyEvents();

    // New order error
    error NewOrderError();

    // Reduce order error
    error ReduceOrderError();

    // Cancel multiple orders error
    error CancelMultipleOrdersError();

    // Withdraw funds error
    error WithdrawFundsError();

    // Remove empty orders error
    error RemoveEmptyOrdersError();

    // Trader not found error
    error TraderNotFound();

    // Invalid seat status
    error InvalidSeatStatus();

    // Failed to evict trader
    error EvictionError();

    // Non empty scratch buffer
    error NonEmptyScratchBuffer();

    // Failed to serialize event
    error FailedToSerializeEvent();

    // Failed to flush buffer
    error FailedToFlushBuffer();
}

#[derive(SolidityError)]
pub enum GoblinError {
    InvalidFeeCollector(InvalidFeeCollector),
    ExceedRestingOrderSize(ExceedRestingOrderSize),
    ExceedTickSize(ExceedTickSize),
    IndicesNotInOrder(IndicesNotInOrder),
    IndexNotInList(IndexNotInList),
    FailedToReduce(FailedToReduce),

    InvalidInstructionData(InvalidInstructionData),
    InvalidMarketParameters(InvalidMarketParameters),
    FailedToLoadMarketFromAccount(FailedToLoadMarketFromAccount),
    MarketAlreadyInitialized(MarketAlreadyInitialized),
    MarketUninitialized(MarketUninitialized),
    InvalidStateTransition(InvalidStateTransition),
    InvalidMarketSigner(InvalidMarketSigner),
    InvalidLotSize(InvalidLotSize),
    InvalidTickSize(InvalidTickSize),
    InvalidMint(InvalidMint),
    InvalidBaseVault(InvalidBaseVault),
    InvalidQuoteVault(InvalidQuoteVault),
    InvalidBaseAccount(InvalidBaseAccount),
    InvalidQuoteAccount(InvalidQuoteAccount),
    TooManyEvents(TooManyEvents),
    NewOrderError(NewOrderError),
    ReduceOrderError(ReduceOrderError),
    CancelMultipleOrdersError(CancelMultipleOrdersError),
    WithdrawFundsError(WithdrawFundsError),
    RemoveEmptyOrdersError(RemoveEmptyOrdersError),
    TraderNotFound(TraderNotFound),
    InvalidSeatStatus(InvalidSeatStatus),
    EvictionError(EvictionError),
    NonEmptyScratchBuffer(NonEmptyScratchBuffer),
    FailedToSerializeEvent(FailedToSerializeEvent),
    FailedToFlushBuffer(FailedToFlushBuffer),
}

#[cfg(test)]
impl Debug for GoblinError {
    fn fmt(&self, _f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Ok(())
    }
}

pub type GoblinResult<T, E = GoblinError> = core::result::Result<T, E>;

#[macro_export]
macro_rules! require {
    ($invariant:expr, $error:expr) => {
        if !$invariant {
            return Err($error);
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_error_bytes() {
        let bytes = Vec::<u8>::from(GoblinError::InvalidInstructionData(
            InvalidInstructionData {},
        ));
        println!("bytes {:?}", bytes);
    }
}
