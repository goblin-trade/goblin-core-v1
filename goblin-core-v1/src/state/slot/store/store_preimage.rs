use crate::{
    axis::{CallerIndexEnum, CallerMarker, CustomCaller, HardcodedCaller, TokenMarker},
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
    pub fn get_hash(&self, token_index: TM::TokenIndex) -> SlotKey<Self> {
        // problem- CallerIndexEnum stores value but CallerEnum does not
        //
        // What does match_axis! do?
        // match enum {
        //   if Hardcoded: func<Hardcoded>()
        // }
        match CallerIndexEnum::from(&self.trader) {
            CallerIndexEnum::HardcodedCaller(caller_index) => {
                let store_key_index = StoreKeyIndex::<HardcodedCaller, TM> {
                    caller_index,
                    token_index,
                };
                let indexed_preimage = IndexedPreimage {
                    store_key_index,
                    preimage: *self,
                };

                HardcodedCaller::get_store_hash(&indexed_preimage)
            }
            CallerIndexEnum::CustomCaller(caller_index) => {
                let store_key_index = StoreKeyIndex::<CustomCaller, TM> {
                    caller_index,
                    token_index,
                };
                let indexed_preimage = IndexedPreimage {
                    store_key_index,
                    preimage: *self,
                };
                CustomCaller::get_store_hash(&indexed_preimage)
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
