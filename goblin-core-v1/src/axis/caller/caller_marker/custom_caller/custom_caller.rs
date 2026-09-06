use crate::{
    axis::{
        caller_marker::custom_caller::CustomCallerStub, CallerMarker, CustomCaller,
        HardcodedCallerIndex, HardcodedCallerList, TokenMarker,
    },
    state::{IndexedPreimage, Preimage, SlotKey, StorePreimage},
    types::Address,
};

impl CallerMarker for CustomCaller {
    type Caller = Address;

    fn get_caller(address: &Address) -> Option<Self::Caller> {
        if HardcodedCallerList::index(address).is_some() {
            None
        } else {
            Some(*address)
        }
    }

    fn get_store_hash<TM: TokenMarker>(
        indexed_preimage: &IndexedPreimage<Self, TM>,
    ) -> SlotKey<StorePreimage<TM>> {
        indexed_preimage.preimage.hash()
    }
}
