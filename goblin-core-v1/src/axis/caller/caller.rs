use crate::{
    axis::{CallerMarker, CustomCallerStub, HardcodedCallerList},
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

impl From<&Address> for CallerEnum {
    fn from(address: &Address) -> Self {
        // TODO fix duplication
        // We find the caller index then discard it. We find it again in TM::get_store_hash()
        //
        // TODO pass CM::CallerIndex = HardcodedCallerIndex or CustomCallerStub
        if HardcodedCallerList::index(address).is_some() {
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

impl From<&Address> for CallerIndexEnum {
    fn from(address: &Address) -> Self {
        if let Some(hardcoded_caller_index) = HardcodedCallerList::index(address) {
            CallerIndexEnum::HardcodedCaller(hardcoded_caller_index)
        } else {
            CallerIndexEnum::CustomCaller(CustomCallerStub)
        }
    }
}
