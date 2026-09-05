use crate::{
    axis::{
        caller_marker::custom_caller::CustomCallerStub, CallerMarker, CustomCaller, TokenMarker,
    },
    state::{Preimage, SlotKey, StorePreimage},
};

impl CallerMarker for CustomCaller {
    type CallerIndex = CustomCallerStub;

    fn get_store_hash<TM: TokenMarker>(
        preimage: &StorePreimage<TM>,
        _token_index: TM::TokenIndex,
        _caller_index: Self::CallerIndex,
    ) -> SlotKey<StorePreimage<TM>> {
        preimage.hash()
    }
}
