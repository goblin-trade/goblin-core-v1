use crate::{
    axis::{CallerMarker, HardcodedCaller, HardcodedCallerIndex, TokenMarker},
    state::{IndexedPreimage, SlotKey, StorePreimage},
};

impl CallerMarker for HardcodedCaller {
    type CallerIndex = HardcodedCallerIndex;

    fn get_caller_index(option_index: Option<HardcodedCallerIndex>) -> Self::CallerIndex {
        // guaranteed to be present in hardcoded caller case
        option_index.unwrap()
    }

    fn get_store_hash<TM: TokenMarker>(
        indexed_preimage: &IndexedPreimage<Self, TM>,
    ) -> SlotKey<StorePreimage<TM>> {
        TM::get_hardcoded_store_hash(indexed_preimage)
    }
}
