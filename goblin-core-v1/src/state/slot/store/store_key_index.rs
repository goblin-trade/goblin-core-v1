use crate::axis::{CallerMarker, TokenMarker};

pub struct StoreKeyIndex<TM: TokenMarker, CM: CallerMarker> {
    pub caller_index: CM::CallerIndex,
    pub token_index: TM::TokenIndex,
}
