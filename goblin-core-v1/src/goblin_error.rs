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
    InvalidLotSize = 15,
    ERC20NotETH = 16,
    InsufficientFreeBalance = 17,
    ShortfallDepositNotAllowed = 18,
    InvalidTokenPair = 19,
    InvalidTakeArgs = 20,
    TakerPriceLimitReached = 21,
    InsufficientTakerFill = 22,
    LocalCounterpartyFull = 23,
    GlobalCounterpartyFull = 24,
    NoInternalSelfWithdraw = 25,
    InvalidTokenPairType = 26,
    InvalidTransferAction = 27,
    InvalidHardcodedMarket = 28,
    InvalidHardcodedTokenIndex = 29,
    InvalidCustomTokenIndex = 30,
    DepositOverflow = 31,
    IteratorOutOfBounds = 32,
    NoRestingOrder = 33,
    UnauthorizedMsgSender = 34,
    InvalidOpenPrice = 35,
    PositionOccupied = 36,
    NoUpdate = 37,
    NoHardcodedDecimals = 38,
    NoHostioDecimals = 39,
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
