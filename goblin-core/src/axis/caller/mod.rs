pub mod caller_marker;

pub use caller_marker::*;

use crate::define_axis;

define_axis! {
    pub struct Caller;
    enum CallerEnum {
        HardcodedCaller = 0,
        CustomCaller = 1,
    }
}

impl From<Option<HardcodedCallerIndex>> for CallerEnum {
    fn from(value: Option<HardcodedCallerIndex>) -> Self {
        if value.is_some() {
            CallerEnum::HardcodedCaller
        } else {
            CallerEnum::CustomCaller
        }
    }
}
