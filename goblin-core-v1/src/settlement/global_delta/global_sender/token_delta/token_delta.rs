use goblin_macros::ConstDefault;

use crate::{
    axis::token::{
        token_marker::{TokenData, TokenMarker},
        token_msg_transfer::TokenMsgTransfer,
        token_quantity::TokenQuantity,
    },
    goblin_error::GoblinError,
    input_processor::MsgTransfers,
    quantities::UnsidedDeltaAtoms,
    state::{Preimage, StorePreimage},
    types::Address,
};

#[derive(Clone, Copy, PartialEq, ConstDefault)]
pub struct TokenDelta<TM: TokenQuantity> {
    pub deposit: TM::GlobalDeposit,
    pub take: UnsidedDeltaAtoms,
    pub make: UnsidedDeltaAtoms,
}

impl<TM: TokenMarker> TokenDelta<TM> {
    pub fn net_delta(&self) -> UnsidedDeltaAtoms {
        self.deposit.into() + self.take + self.make
    }

    pub fn settle(
        &self,
        trader: &Address,
        token_data: &TokenData<TM>,
        msg_transfers: &MsgTransfers,
    ) -> Result<(), GoblinError> {
        let msg_transfer = TM::get_leg(msg_transfers);

        // 1. Update store
        let store_hash = StorePreimage::<TM> {
            trader: *trader,
            token_address: token_data.address,
        }
        .hash();
        let mut store = store_hash.load();
        store.update_sender(token_data, self, msg_transfer)?;
        store_hash.store(&store);

        // 2. Transfer tokens
        let net_deposit = self.deposit.into() + msg_transfer.deposit_due()?;

        TM::transfer(net_deposit, trader, &token_data.address, store.decimals)
    }
}
