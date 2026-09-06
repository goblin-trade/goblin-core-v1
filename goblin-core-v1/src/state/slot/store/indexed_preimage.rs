use crate::{
    axis::{CallerMarker, TokenMarker},
    state::{StoreKeyIndex, StorePreimage},
    types::Address,
};

pub struct IndexedPreimage<TM: TokenMarker, CM: CallerMarker> {
    pub store_key_index: StoreKeyIndex<TM, CM>,
    pub preimage: StorePreimage<TM>,
}
