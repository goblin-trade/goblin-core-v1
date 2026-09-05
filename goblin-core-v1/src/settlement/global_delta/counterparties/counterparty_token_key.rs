use goblin_macros::ConstDefault;

use crate::{
    axis::token::{
        token_marker::TokenMarker, token_quantity::TokenQuantity, token_reader::TokenDataTriple,
    },
    state::{Preimage, SlotKey, StorePreimage},
    types::Address,
};

#[derive(PartialEq, Clone, Copy, ConstDefault)]
pub struct CounterpartyTokenKey<TM: TokenQuantity> {
    pub counterparty: Address,
    pub token_index: TM::TokenIndex,
}

impl<TM: TokenMarker> CounterpartyTokenKey<TM> {
    pub fn get_store_hash(
        &self,
        token_data_triple: &TokenDataTriple,
    ) -> SlotKey<StorePreimage<TM>> {
        let token_data_list = TM::get_lifetimed(token_data_triple);
        let token_data = token_data_list[self.token_index];

        // TODO use Caller axis to get hash
        // Hash is hardcoded for HardcodedCaller + (HardcodedERC20 or ERC20)
        let preimage = StorePreimage {
            trader: self.counterparty,
            token_address: token_data.address,
        };

        preimage.hash()
    }
}
