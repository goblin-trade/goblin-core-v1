use crate::{
    axis::{caller::CallerMarker, token::token_marker::TokenMarker},
    state::{ConstPreimage, IndexedPreimage, Preimage, SlotKey, Store, StoreKeyIndex},
    types::Address,
};

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct StorePreimage<TM: TokenMarker> {
    pub trader: Address,
    pub token_address: TM::TokenAddress,
}

impl<TM: TokenMarker> StorePreimage<TM> {
    pub fn get_hash<CM: CallerMarker>(
        &self,
        store_key_index: StoreKeyIndex<CM, TM>,
    ) -> SlotKey<Self> {
        let indexed_preimage = IndexedPreimage {
            store_key_index,
            preimage: *self,
        };

        CM::get_store_hash(&indexed_preimage)
    }
}

impl<TM: TokenMarker> Preimage for StorePreimage<TM> {
    const SLOT_DISCRIMINATOR: u8 = 2 + TM::DISCRIMINATOR;
    type SlotState = Store<TM>;
}

const impl<TM: TokenMarker> ConstPreimage for StorePreimage<TM> {}
