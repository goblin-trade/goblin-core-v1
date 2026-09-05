use crate::{
    axis::{
        CallerEnum, CallerIndexEnum, CallerMarker, CustomCaller, HardcodedCaller,
        HardcodedCallerIndex, TokenMarker,
    },
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
    pub fn get_hash(&self, token_index: TM::TokenIndex) -> SlotKey<Self> {
        match CallerIndexEnum::from(&self.trader) {
            CallerIndexEnum::HardcodedCaller(caller_index) => {
                TM::get_store_hash::<HardcodedCaller>(self, token_index, caller_index)
            }
            CallerIndexEnum::CustomCaller(caller_index) => {
                TM::get_store_hash::<CustomCaller>(self, token_index, caller_index)
            }
        }
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
