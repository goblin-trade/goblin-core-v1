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
    InvalidTokenPair = 18,
    InvalidTakeArgs = 19,
    TakerPriceLimitReached = 20,
    InsufficientTakerFill = 21,
    LocalCounterpartyFull = 22,
    GlobalCounterpartyFull = 23,
    NoInternalSelfWithdraw = 24,
    InvalidTokenPairType = 25,
    InvalidTransferAction = 26,
    InvalidHardcodedMarket = 27,
    InvalidHardcodedTokenIndex = 28,
    InvalidCustomTokenIndex = 29,
    DepositOverflow = 30,
    IteratorOutOfBounds = 31,
    NoRestingOrder = 32,
    UnauthorizedMsgSender = 33,
    InvalidOpenPrice = 34,
    PositionOccupied = 35,
    NoUpdate = 36,
    NoHardcodedDecimals = 37,
    NoHostioDecimals = 38,
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
