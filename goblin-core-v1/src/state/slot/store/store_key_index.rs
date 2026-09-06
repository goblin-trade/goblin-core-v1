use crate::axis::{CallerMarker, TokenMarker};

pub struct StoreKeyIndex<CM: CallerMarker, TM: TokenMarker> {
    pub caller_index: CM::Locator,
    pub token_index: TM::TokenIndex,
}
