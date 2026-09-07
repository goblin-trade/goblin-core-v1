use crate::{
    axis::{CallerMarker, HardcodedCaller, HardcodedCallerIndex, TokenMarker},
    state::{IndexedPreimage, SlotKey, StorePreimage},
};

impl CallerMarker for HardcodedCaller {
    type Locator = HardcodedCallerIndex;

    fn get_locator(maybe_hardcoded_caller_index: Option<HardcodedCallerIndex>) -> Self::Locator {
        maybe_hardcoded_caller_index.unwrap()
    }

    fn get_store_hash<TM: TokenMarker>(
        indexed_preimage: &IndexedPreimage<Self, TM>,
    ) -> SlotKey<StorePreimage<TM>> {
        TM::get_hardcoded_store_hash(indexed_preimage)
    }
}
