use crate::{
    axis::{
        party::{PartySettle, Sender},
        token::{token_marker::TokenMarker, token_reader::TokenDataTriple},
        CallerData, CallerEnum, CallerMarker, HardcodedCallerList,
    },
    goblin_error::GoblinError,
    input_processor::MsgTransfers,
    match_axes,
    settlement::{
        global_delta::{GlobalDelta, TokenDelta},
        ConstDefault, TokenSettler,
    },
    types::{Address, StoreReader},
};

impl PartySettle for Sender {
    fn settle<'a, TM>(
        global_delta: &'a GlobalDelta,
        token_data_triple: &TokenDataTriple<'a>,
        msg_transfers: &MsgTransfers,
        caller: &Address,
        recipient: &Address,
    ) -> Result<(), GoblinError>
    where
        TM: TokenMarker,
        &'a TM::SenderDeltaList: IntoIterator<Item = &'a TokenDelta<TM>>,
    {
        let sender_delta = Self::get_leg(global_delta);

        let data_iter = TM::get_lifetimed(&token_data_triple).into_iter();
        let delta_iter = TM::get_leg(sender_delta).into_iter();

        let maybe_hardcoded_caller_index = HardcodedCallerList::index(caller);
        let caller_enum = CallerEnum::from(maybe_hardcoded_caller_index);

        match_axes!(CM = caller_enum => {
            let caller_data = CallerData::<CM> {
                address: caller,
                locator: CM::get_locator(maybe_hardcoded_caller_index)
            };

            for (index, (token_data, token_delta)) in
                data_iter.zip(delta_iter).enumerate()
            {
                if *token_delta != TokenDelta::DEFAULT {
                    let token_index = TM::TokenIndex::from(index);

                    TokenSettler {
                        token_index,
                        token_data,
                        token_delta,
                        caller_data,
                        msg_transfers,
                        recipient,
                    }
                    .settle()?;
                }
            }
        });

        Ok(())
    }
}
