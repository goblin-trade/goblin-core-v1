use crate::axis::{CallerMarker, TokenMarker};

pub struct StoreKeyIndex<CM: CallerMarker, TM: TokenMarker> {
    pub caller_index: CM::Caller,
    pub token_index: TM::TokenIndex,
}
