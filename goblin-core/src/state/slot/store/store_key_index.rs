use crate::axis::{caller::CallerMarker, token::token_marker::TokenMarker};

pub struct StoreKeyIndex<CM: CallerMarker, TM: TokenMarker> {
    pub caller_locator: CM::Locator,
    pub token_index: TM::TokenIndex,
}
