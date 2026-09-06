use crate::{
    axis::{CallerMarker, HardcodedCaller, HardcodedCallerIndex, HardcodedCallerList, TokenMarker},
    state::{IndexedPreimage, SlotKey, StorePreimage},
    types::Address,
};

impl CallerMarker for HardcodedCaller {
    type Locator = HardcodedCallerIndex;

    fn get_locator(address: &Address) -> Option<Self::Locator> {
        HardcodedCallerList::index(address)
    }

    fn get_store_hash<TM: TokenMarker>(
        indexed_preimage: &IndexedPreimage<Self, TM>,
    ) -> SlotKey<StorePreimage<TM>> {
        TM::get_hardcoded_store_hash(indexed_preimage)
    }
}
