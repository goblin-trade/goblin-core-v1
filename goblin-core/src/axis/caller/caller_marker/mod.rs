pub mod custom_caller;
pub mod hardcoded_caller;

pub use custom_caller::*;
pub use hardcoded_caller::*;

use crate::{
    axis::{caller::CallerEnum, token::token_marker::TokenMarker},
    axis_helpers::AxisMarker,
    state::{IndexedPreimage, SlotKey, StorePreimage},
};

pub trait CallerMarker: AxisMarker<Enum = CallerEnum> {
    type Locator: Clone + Copy;

    fn get_locator(maybe_hardcoded_caller_index: Option<HardcodedCallerIndex>) -> Self::Locator;

    fn get_store_hash<TM: TokenMarker>(
        indexed_preimage: &IndexedPreimage<Self, TM>,
    ) -> SlotKey<StorePreimage<TM>>;
}
