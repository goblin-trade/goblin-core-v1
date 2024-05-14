use alloy_sol_types::sol;
use stylus_sdk::prelude::*;

sol! {
    // Invalid instruction data error
    error InvalidInstructionData();

    // Invalid market parameters error
    error InvalidMarketParameters();

    // Invalid market authority error
    error InvalidMarketAuthority();

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
    InvalidInstructionData(InvalidInstructionData),
    InvalidMarketParameters(InvalidMarketParameters),
    InvalidMarketAuthority(InvalidMarketAuthority),
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

pub type GoblinResult<T, E = GoblinError> = core::result::Result<T, E>;

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
