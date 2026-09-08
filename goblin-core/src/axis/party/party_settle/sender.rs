use crate::{
    axis::{
        caller::{CallerEnum, CallerMarker, HardcodedCallerList},
        party::{PartySettle, Sender},
        token::{token_marker::TokenMarker, token_reader::TokenDataTriple},
    },
    goblin_error::GoblinError,
    input_processor::{CallerAddresses, MsgTransfers},
    match_axes,
    settlement::{
        ConstDefault, TokenSettler,
        global_delta::{GlobalDelta, TokenDelta},
    },
    types::StoreReader,
};

impl PartySettle for Sender {
    fn settle<'a, TM>(
        global_delta: &'a GlobalDelta,
        token_data_triple: &TokenDataTriple<'a>,
        msg_transfers: &MsgTransfers,
        caller_addresses: CallerAddresses<'a>,
    ) -> Result<(), GoblinError>
    where
        TM: TokenMarker,
        &'a TM::SenderDeltaList: IntoIterator<Item = &'a TokenDelta<TM>>,
    {
        let sender_delta = Self::get_leg(global_delta);

        let data_iter = TM::get_lifetimed(token_data_triple).into_iter();
        let delta_iter = TM::get_leg(sender_delta).into_iter();

        let maybe_hardcoded_caller_index = HardcodedCallerList::index(caller_addresses.caller);
        let caller_enum = CallerEnum::from(maybe_hardcoded_caller_index);

        match_axes!(CM = caller_enum => {
            let locator = CM::get_locator(maybe_hardcoded_caller_index);

            for (index, (token_data, token_delta)) in
                data_iter.zip(delta_iter).enumerate()
            {
                if *token_delta != TokenDelta::DEFAULT {
                    let token_index = TM::TokenIndex::from(index);

                    TokenSettler::<CM, TM> {
                        token_index,
                        locator,
                        token_data,
                        token_delta,
                        msg_transfers,
                        caller_addresses,
                    }
                    .settle()?;
                }
            }
        });

        Ok(())
    }
}
