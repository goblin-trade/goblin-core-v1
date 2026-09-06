use crate::{
    axis::{CallerMarker, TokenMarker},
    state::{StoreKeyIndex, StorePreimage},
};

pub struct IndexedPreimage<CM: CallerMarker, TM: TokenMarker> {
    pub store_key_index: StoreKeyIndex<CM, TM>,
    pub preimage: StorePreimage<TM>,
}
