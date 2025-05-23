#[derive(Debug)]
pub enum GoblinError {
    Reentrant = 0,
    InvalidPayload = 1,
    InvalidSelector = 2,
    UnsupportedDecimals = 3,
    DecimalReadFail = 4,
    CallFail = 5,
    CallResultInvalid = 6,
    TraderTokenStateEmpty = 7,
    DeltaListFull = 8,
    AtomOverflow = 9,
    AtomUndeflow = 10,
    DeltaOverflow = 11,
    DeltaUnderflow = 12,
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
