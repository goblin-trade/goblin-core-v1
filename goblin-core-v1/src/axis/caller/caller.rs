use crate::{axis::HARDCODED_CALLERS, define_axis, types::Address};

define_axis! {
    pub struct Caller;
    enum CallerEnum {
        HardcodedCaller = 0,
        CustomCaller = 1,
    }
}

impl From<&Address> for CallerEnum {
    fn from(address: &Address) -> Self {
        if *address == HARDCODED_CALLERS[0] || *address == HARDCODED_CALLERS[1] {
            CallerEnum::HardcodedCaller
        } else {
            CallerEnum::CustomCaller
        }
    }
}

impl From<Address> for CallerEnum {
    fn from(address: Address) -> Self {
        Self::from(&address)
    }
}
