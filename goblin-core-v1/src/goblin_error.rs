#[derive(Debug)]
#[repr(u8)]
pub enum GoblinError {
    Reentrant = 0,
    InvalidPayload = 1,
    UnsupportedDecimals = 2,
    InvalidLotSize = 3,
    InvalidTakeArgs = 4,
    TakerPriceLimitReached = 5,
    LocalCounterpartyFull = 6,
    DeltaOverflow = 7,
    InsufficientTakerFill = 8,
    NoRestingOrder = 9,
    InvalidOpenPrice = 10,
    PositionOccupied = 11,
    UnauthorizedMsgSender = 12,
    Overflow = 13,
    Underflow = 14,
    NegativeValue = 15,
    GlobalCounterpartyFull = 16,
    StaticCallFail = 17,
    CallFail = 18,
    CallResultInvalid = 19,
    NoHardcodedDecimals = 20,
    NoHostioDecimals = 21,
    InvalidEnumVariant = 22,
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
