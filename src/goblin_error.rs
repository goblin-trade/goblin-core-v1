#[derive(Debug)]
pub enum GoblinError {
    InvalidPayload = 1,
    InvalidSelector = 2,
    UnsupportedDecimals = 3,
    DecimalReadFail = 4,
    CallFail = 5,
    CallResultInvalid = 6,
    TraderTokenStateEmpty = 7,
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
