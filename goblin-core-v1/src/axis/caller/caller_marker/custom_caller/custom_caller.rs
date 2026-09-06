use crate::{
    axis::{
        caller_marker::custom_caller::CustomCallerStub, CallerMarker, CustomCaller,
        HardcodedCallerList, TokenMarker,
    },
    state::{IndexedPreimage, Preimage, SlotKey, StorePreimage},
    types::Address,
};

impl CallerMarker for CustomCaller {
    type Locator = CustomCallerStub;

    fn get_locator(address: &Address) -> Option<Self::Locator> {
        if HardcodedCallerList::index(address).is_some() {
            None
        } else {
            Some(CustomCallerStub)
        }
    }

    fn get_store_hash<TM: TokenMarker>(
        indexed_preimage: &IndexedPreimage<Self, TM>,
    ) -> SlotKey<StorePreimage<TM>> {
        indexed_preimage.preimage.hash()
    }
}
