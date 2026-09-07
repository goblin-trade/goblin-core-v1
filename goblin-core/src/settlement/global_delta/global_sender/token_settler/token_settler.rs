use crate::{
    axis::{CallerMarker, TokenData, TokenMarker, TokenMsgTransfer, TokenQuantity},
    goblin_error::GoblinError,
    input_processor::{CallerAddresses, MsgTransfers},
    settlement::{transfer_token, TokenDelta},
    state::{IndexedPreimage, StoreKeyIndex, StorePreimage},
};

pub struct TokenSettler<'a, CM: CallerMarker, TM: TokenQuantity> {
    pub token_index: TM::TokenIndex,
    pub locator: CM::Locator,
    pub token_data: &'a TokenData<TM>,
    pub token_delta: &'a TokenDelta<TM>,
    pub msg_transfers: &'a MsgTransfers,
    pub caller_addresses: CallerAddresses<'a>,
}

impl<'a, CM: CallerMarker, TM: TokenMarker> TokenSettler<'a, CM, TM> {
    pub fn settle(&self) -> Result<(), GoblinError> {
        let msg_transfer = TM::get_leg(self.msg_transfers);

        let indexed_preimage = IndexedPreimage {
            store_key_index: StoreKeyIndex::<CM, TM> {
                caller_locator: self.locator,
                token_index: self.token_index,
            },
            preimage: StorePreimage {
                trader: *self.caller_addresses.caller,
                token_address: self.token_data.address,
            },
        };

        // 1. Update store for caller
        let store_hash = CM::get_store_hash(&indexed_preimage);

        let mut store = store_hash.load();
        store.update_sender(self.token_data, self.token_delta, msg_transfer)?;
        store_hash.store(&store);

        // 2. Transfer tokens to recipient
        let net_deposit = self.token_delta.deposit.into() + msg_transfer.deposit_due()?;

        transfer_token::<TM>(
            net_deposit,
            &self.token_data.address,
            store.decimals,
            self.caller_addresses,
        )
    }
}
