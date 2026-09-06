use crate::{
    axis::{CallerData, CallerMarker, TokenData, TokenMarker, TokenMsgTransfer, TokenQuantity},
    goblin_error::GoblinError,
    input_processor::MsgTransfers,
    settlement::TokenDelta,
    state::{IndexedPreimage, StoreKeyIndex, StorePreimage},
    types::Address,
};

pub struct TokenSettler<'a, CM: CallerMarker, TM: TokenQuantity> {
    pub token_index: TM::TokenIndex,
    pub token_data: &'a TokenData<TM>,
    pub token_delta: &'a TokenDelta<TM>,
    pub caller_data: CallerData<'a, CM>,
    pub msg_transfers: &'a MsgTransfers,
    pub recipient: &'a Address,
}

impl<'a, CM: CallerMarker, TM: TokenMarker> TokenSettler<'a, CM, TM> {
    pub fn settle(&self) -> Result<(), GoblinError> {
        let msg_transfer = TM::get_leg(self.msg_transfers);

        let indexed_preimage = IndexedPreimage {
            store_key_index: StoreKeyIndex {
                caller_locator: self.caller_data.locator,
                token_index: self.token_index,
            },
            preimage: StorePreimage::<TM> {
                trader: *self.caller_data.address,
                token_address: self.token_data.address,
            },
        };

        // 1. Update store for caller
        let store_hash = CM::get_store_hash(&indexed_preimage);

        let mut store = store_hash.load();
        store.update_sender(self.token_data, self.token_delta, msg_transfer)?;
        store_hash.store(&store);

        // 2. Transfer tokens to recipient
        //
        // TODO fix- we can only send surplus to recipient, not withdraw shortfall
        let net_deposit = self.token_delta.deposit.into() + msg_transfer.deposit_due()?;

        TM::transfer(
            net_deposit,
            self.recipient,
            &self.token_data.address,
            store.decimals,
        )
    }
}
