use goblin_macros::ConstDefault;

use crate::{
    axis::token::{
        token_marker::TokenMarker, token_quantity::TokenQuantity, token_reader::TokenDataTriple,
    },
    state::{Preimage, SlotKey, StorePreimage},
    types::Address,
};

#[derive(PartialEq, Clone, Copy, ConstDefault)]
pub struct CounterpartyTokenKey<T: TokenQuantity> {
    pub counterparty: Address,
    pub token_index: T::TokenIndex,
}

impl<T: TokenMarker> CounterpartyTokenKey<T> {
    pub fn get_store_hash(&self, token_data_triple: &TokenDataTriple) -> SlotKey<StorePreimage<T>> {
        let token_data_list = T::get_lifetimed(token_data_triple);
        let token_data = token_data_list[self.token_index];

        let preimage = StorePreimage {
            trader: self.counterparty,
            token_address: token_data.address,
        };

        preimage.hash()
    }
}
