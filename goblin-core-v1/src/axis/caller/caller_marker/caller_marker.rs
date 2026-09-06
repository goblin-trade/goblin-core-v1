use crate::{
    axis::{CallerEnum, HardcodedCallerIndex, TokenMarker},
    axis_helpers::AxisMarker,
    state::{IndexedPreimage, SlotKey, StorePreimage},
};

pub trait CallerMarker: AxisMarker<Enum = CallerEnum> {
    type CallerIndex: Clone + Copy;

    fn get_caller_index(option_index: Option<HardcodedCallerIndex>) -> Self::CallerIndex;

    fn get_store_hash<TM: TokenMarker>(
        indexed_preimage: &IndexedPreimage<Self, TM>,
    ) -> SlotKey<StorePreimage<TM>>;
}
