use crate::{
    axis::{CallerEnum, TokenMarker},
    axis_helpers::AxisMarker,
    state::{SlotKey, StorePreimage},
};

pub trait CallerMarker: AxisMarker<Enum = CallerEnum> {
    type CallerIndex;

    fn get_store_hash<TM: TokenMarker>(
        preimage: &StorePreimage<TM>,
        _token_index: TM::TokenIndex,
        caller_index: Self::CallerIndex,
    ) -> SlotKey<StorePreimage<TM>>;
}
