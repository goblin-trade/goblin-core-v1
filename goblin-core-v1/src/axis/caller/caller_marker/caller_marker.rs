use crate::{
    axis::{CallerEnum, TokenMarker},
    axis_helpers::AxisMarker,
    state::{IndexedPreimage, SlotKey, StorePreimage},
};

pub trait CallerMarker: AxisMarker<Enum = CallerEnum> {
    type CallerIndex: Clone + Copy;

    fn get_store_hash<TM: TokenMarker>(
        indexed_preimage: &IndexedPreimage<Self, TM>,
    ) -> SlotKey<StorePreimage<TM>>;
}
