pub mod custom_caller_stub;
pub use custom_caller_stub::*;

use crate::{
    axis::{
        caller::{CallerMarker, CustomCaller, HardcodedCallerIndex},
        token::token_marker::TokenMarker,
    },
    state::{IndexedPreimage, Preimage, SlotKey, StorePreimage},
};

impl CallerMarker for CustomCaller {
    type Locator = CustomCallerStub;

    fn get_locator(_maybe_hardcoded_caller_index: Option<HardcodedCallerIndex>) -> Self::Locator {
        CustomCallerStub
    }

    fn get_store_hash<TM: TokenMarker>(
        indexed_preimage: &IndexedPreimage<Self, TM>,
    ) -> SlotKey<StorePreimage<TM>> {
        indexed_preimage.preimage.hash()
    }
}
