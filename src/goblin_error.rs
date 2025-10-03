#[derive(Debug)]
#[repr(u8)]
pub enum GoblinError {
    Reentrant = 0,
    InvalidPayload = 1,
    CustomTokenLimitExceeded = 2,
    UnsupportedDecimals = 3,
    StaticCallFail = 4,
    CallFail = 5,
    CallResultInvalid = 6,
    TraderTokenStateEmpty = 7,
    ERC20DeltaListFull = 8,
    Overflow = 9,
    Underflow = 10,
    DeltaOverflow = 11,
    DeltaUnderflow = 12,
    NoTokenAtIndex = 13,
    ERC20NotETH = 14,
    InsufficientFreeBalance = 15,
    ShortfallDepositNotAllowed = 16,
    NoMarketAtIndex = 17,
    InvalidMarket = 18,
    InvalidTokenPair = 19,
    InvalidTakeArgs = 20,
    TakerPriceLimitReached = 21,
    InsufficientTakerFill = 22,
    MakerListFull = 23,
    MakerStoreListFull = 24,
    NoInternalSelfWithdraw = 25,
    InvalidTokenPairType = 26,
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
