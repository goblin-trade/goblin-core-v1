use crate::{
    axis::{
        caller_marker::custom_caller::CustomCallerStub, CallerMarker, CustomCaller,
        HardcodedCallerIndex, TokenMarker,
    },
    state::{IndexedPreimage, Preimage, SlotKey, StorePreimage},
};

impl CallerMarker for CustomCaller {
    type CallerIndex = CustomCallerStub;

    fn get_caller_index(_option_index: Option<HardcodedCallerIndex>) -> Self::CallerIndex {
        CustomCallerStub
    }

    fn get_store_hash<TM: TokenMarker>(
        indexed_preimage: &IndexedPreimage<Self, TM>,
    ) -> SlotKey<StorePreimage<TM>> {
        indexed_preimage.preimage.hash()
    }
}
