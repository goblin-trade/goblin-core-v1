#[derive(Debug)]
pub enum GoblinError {
    Reentrant = 0,
    InvalidPayload = 1,
    CustomTokenLimitExceeded = 2,
    InvalidSelector = 3,
    UnsupportedDecimals = 4,
    DecimalReadFail = 5,
    CallFail = 6,
    CallResultInvalid = 7,
    TraderTokenStateEmpty = 8,
    DeltaListFull = 9,
    AtomOverflow = 10,
    AtomUndeflow = 11,
    DeltaOverflow = 12,
    DeltaUnderflow = 13,
    CannotDepositEthOnSettlement = 14,
    NoTokenAtIndex = 15,
    InsufficientFreeBalance = 16,
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
