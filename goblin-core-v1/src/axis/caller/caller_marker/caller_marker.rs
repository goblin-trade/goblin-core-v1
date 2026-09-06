use crate::{
    axis::{CallerEnum, HardcodedCallerIndex, TokenMarker},
    axis_helpers::AxisMarker,
    state::{IndexedPreimage, SlotKey, StorePreimage},
};

pub trait CallerMarker: AxisMarker<Enum = CallerEnum> {
    type Caller: Clone + Copy;

    fn get_caller(maybe_hardcded_caller: Option<HardcodedCallerIndex>) -> Self::Caller;

    fn get_store_hash<TM: TokenMarker>(
        indexed_preimage: &IndexedPreimage<Self, TM>,
    ) -> SlotKey<StorePreimage<TM>>;
}
