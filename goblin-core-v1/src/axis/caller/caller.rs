use crate::{
    axis::{CallerMarker, CustomCallerStub, HardcodedCallerIndex, HardcodedCallerList},
    define_axis,
    types::Address,
};

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

pub enum CallerIndexEnum {
    HardcodedCaller(<HardcodedCaller as CallerMarker>::CallerIndex),
    CustomCaller(<CustomCaller as CallerMarker>::CallerIndex),
}

impl CallerIndexEnum {
    pub fn kind(&self) -> CallerEnum {
        match self {
            Self::HardcodedCaller(_) => CallerEnum::HardcodedCaller,
            Self::CustomCaller(_) => CallerEnum::CustomCaller,
        }
    }
}

impl From<&Address> for CallerIndexEnum {
    fn from(address: &Address) -> Self {
        if let Some(hardcoded_caller_index) = HardcodedCallerList::index(address) {
            CallerIndexEnum::HardcodedCaller(hardcoded_caller_index)
        } else {
            CallerIndexEnum::CustomCaller(CustomCallerStub)
        }
    }
}
