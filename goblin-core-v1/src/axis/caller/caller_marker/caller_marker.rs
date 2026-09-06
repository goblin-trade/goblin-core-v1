use crate::{
    axis::{CallerEnum, TokenMarker},
    axis_helpers::AxisMarker,
    state::{IndexedPreimage, SlotKey, StorePreimage},
    types::Address,
};

pub trait CallerMarker: AxisMarker<Enum = CallerEnum> {
    type Caller: Clone + Copy;

    fn get_caller(address: &Address) -> Option<Self::Caller>;

    fn get_store_hash<TM: TokenMarker>(
        indexed_preimage: &IndexedPreimage<Self, TM>,
    ) -> SlotKey<StorePreimage<TM>>;
}
