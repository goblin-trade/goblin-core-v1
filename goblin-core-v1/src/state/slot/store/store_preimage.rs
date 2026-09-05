use crate::{
    axis::{CallerEnum, CallerMarker, TokenMarker},
    match_axes,
    state::{Preimage, PreimageSerializer, SlotKey, Store},
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
    pub fn get_hash(&self, token_index: &TM::TokenIndex) -> SlotKey<Self> {
        let caller_enum = CallerEnum::from(&self.trader);
        match_axes!(CM = caller_enum => {
            Self::get_store_hash::<CM>(self, token_index)
        })
    }

    pub fn get_store_hash<CM: CallerMarker>(&self, token_index: &TM::TokenIndex) -> SlotKey<Self> {
        TM::get_store_hash::<CM>(self, token_index)
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
