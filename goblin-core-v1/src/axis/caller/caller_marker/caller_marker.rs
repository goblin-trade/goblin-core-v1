use crate::{
    axis::{CallerEnum, TokenMarker},
    axis_helpers::AxisMarker,
    state::{IndexedPreimage, SlotKey, StorePreimage},
    types::Address,
};

pub trait CallerMarker: AxisMarker<Enum = CallerEnum> {
    type Locator: Clone + Copy;

    fn get_locator(address: &Address) -> Option<Self::Locator>;

    fn get_store_hash<TM: TokenMarker>(
        indexed_preimage: &IndexedPreimage<Self, TM>,
    ) -> SlotKey<StorePreimage<TM>>;
}
