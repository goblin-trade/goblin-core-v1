use crate::{axis::HardcodedCallerList, define_axis, types::Address};

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

impl From<Address> for CallerEnum {
    fn from(address: Address) -> Self {
        Self::from(&address)
    }
}
