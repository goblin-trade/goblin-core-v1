use crate::{
    axis::{CallerMarker, HardcodedCaller, HardcodedCallerIndex, TokenMarker},
    state::{IndexedPreimage, SlotKey, StorePreimage},
};

impl CallerMarker for HardcodedCaller {
    type Caller = HardcodedCallerIndex;

    fn get_caller(_maybe_hardcded_caller: Option<HardcodedCallerIndex>) -> Self::Caller {
        // guaranteed to be present in hardcoded caller case
        _maybe_hardcded_caller.unwrap()
    }

    fn get_store_hash<TM: TokenMarker>(
        indexed_preimage: &IndexedPreimage<Self, TM>,
    ) -> SlotKey<StorePreimage<TM>> {
        TM::get_hardcoded_store_hash(indexed_preimage)
    }
}
