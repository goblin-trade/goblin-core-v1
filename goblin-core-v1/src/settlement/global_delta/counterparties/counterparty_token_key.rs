use goblin_macros::ConstDefault;

use crate::{
    axis::{
        token::{
            token_marker::TokenMarker, token_quantity::TokenQuantity, token_reader::TokenDataTriple,
        },
        CallerMarker,
    },
    state::{IndexedPreimage, SlotKey, StoreKeyIndex, StorePreimage},
    types::Address,
};

#[derive(PartialEq, Clone, Copy, ConstDefault)]
pub struct CounterpartyTokenKey<TM: TokenQuantity> {
    pub counterparty: Address,
    pub token_index: TM::TokenIndex,
}

impl<TM: TokenMarker> CounterpartyTokenKey<TM> {
    pub fn get_store_hash<CM: CallerMarker>(
        &self,
        locator: CM::Locator,
        token_data_triple: &TokenDataTriple,
    ) -> SlotKey<StorePreimage<TM>> {
        let token_data_list = TM::get_lifetimed(token_data_triple);
        let token_data = token_data_list[self.token_index];

        let indexed_preimage = IndexedPreimage {
            store_key_index: StoreKeyIndex {
                caller_locator: locator,
                token_index: self.token_index,
            },
            preimage: StorePreimage::<TM> {
                trader: self.counterparty,
                token_address: token_data.address,
            },
        };

        CM::get_store_hash(&indexed_preimage)
    }
}
