use crate::{
    axis::{
        party::{PartySettle, Sender},
        token::{token_marker::TokenMarker, token_reader::TokenDataTriple},
        CallerData, CallerMarker,
    },
    goblin_error::GoblinError,
    input_processor::MsgTransfers,
    settlement::{
        global_delta::{GlobalDelta, TokenDelta},
        ConstDefault,
    },
    types::{Address, StoreReader},
};

impl PartySettle for Sender {
    fn settle<'a, TM, CM>(
        caller_data: CallerData<'a, CM>,
        recipient: &Address,
        global_delta: &'a GlobalDelta,
        token_data_triple: &TokenDataTriple<'a>,
        transfers: &MsgTransfers,
    ) -> Result<(), GoblinError>
    where
        TM: TokenMarker,
        &'a TM::SenderDeltaList: IntoIterator<Item = &'a TokenDelta<TM>>,
        CM: CallerMarker,
    {
        let sender_delta = Self::get_leg(global_delta);

        let data_list_iter = TM::get_lifetimed(&token_data_triple).into_iter();
        let sender_delta_list_iter = TM::get_leg(sender_delta).into_iter();

        for (index, (token_data, delta)) in data_list_iter.zip(sender_delta_list_iter).enumerate() {
            if *delta != TokenDelta::DEFAULT {
                let token_index = TM::TokenIndex::from(index);
                delta.settle(recipient, &token_data, token_index, transfers)?;
            }
        }

        Ok(())
    }
}
