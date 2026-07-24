#[derive(Debug)]
#[repr(u8)]
pub enum GoblinError {
    Reentrant = 0,
    InvalidPayload = 1,
    CustomERC20CountExceeded = 2,
    UnsupportedDecimals = 3,
    StaticCallFail = 4,
    CallFail = 5,
    CallResultInvalid = 6,
    TraderTokenStateEmpty = 7,
    ERC20DeltaListFull = 8,
    Overflow = 9,
    Underflow = 10,
    NegativeValue = 11,
    DeltaOverflow = 12,
    DeltaUnderflow = 13,
    NoTokenAtIndex = 14,
    ERC20NotETH = 15,
    InsufficientFreeBalance = 16,
    ShortfallDepositNotAllowed = 17,
    NoMarketAtIndex = 18,
    InvalidMarket = 19,
    InvalidTokenPair = 20,
    InvalidTakeArgs = 21,
    TakerPriceLimitReached = 22,
    InsufficientTakerFill = 23,
    LocalCounterpartyFull = 24,
    GlobalCounterpartyFull = 25,
    NoInternalSelfWithdraw = 26,
    InvalidTokenPairType = 27,
    InvalidTransferAction = 28,
    InvalidHardcodedMarket = 29,
    InvalidHardcodedTokenIndex = 30,
    InvalidCustomTokenIndex = 31,
    DepositOverflow = 32,
    IteratorOutOfBounds = 33,
    NoRestingOrder = 34,
    UnauthorizedMsgSender = 35,
    InvalidOpenPrice = 36,
    PositionOccupied = 37,
    NoUpdate = 38,
    NoHardcodedDecimals = 39,
    NoHostioDecimals = 40,
}

impl GoblinError {
    pub fn code(self) -> i32 {
        self as i32
    }
}

#[macro_export]
macro_rules! require {
    ($cond:expr, $err:expr) => {
        if !$cond {
            return Err($err);
        }
    };
}
