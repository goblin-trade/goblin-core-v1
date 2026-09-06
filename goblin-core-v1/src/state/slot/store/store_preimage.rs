use crate::{
    axis::{CallerMarker, TokenMarker},
    state::{IndexedPreimage, Preimage, PreimageSerializer, SlotKey, Store, StoreKeyIndex},
    types::Address,
};
use keccak_const::Keccak256;

#[repr(C)]
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

    pub const fn const_hash(&self) -> SlotKey<Self> {
        let buffer = PreimageSerializer::new(*self);
        let bytes = buffer.serialize();
        let hash = Keccak256::new().update(bytes).finalize();

        SlotKey::<Self>::new(hash)
    }
}

impl<TM: TokenMarker> Preimage for StorePreimage<TM> {
    const SLOT_DISCRIMINATOR: u8 = 2 + TM::DISCRIMINATOR;
    type SlotState = Store<TM>;
}
