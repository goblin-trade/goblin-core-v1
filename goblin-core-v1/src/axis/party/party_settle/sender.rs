use crate::{
    axis::{
        party::{PartySettle, Sender},
        token::{token_marker::TokenMarker, token_reader::TokenDataTriple},
        CallerEnum, CallerMarker, CustomCaller, CustomCallerStub, HardcodedCallerList,
    },
    goblin_error::GoblinError,
    input_processor::{CallerAddresses, MsgTransfers},
    match_axes,
    settlement::{
        global_delta::{GlobalDelta, TokenDelta},
        ConstDefault, TokenSettler,
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

        let data_iter = TM::get_lifetimed(&token_data_triple).into_iter();
        let delta_iter = TM::get_leg(sender_delta).into_iter();

        let maybe_hardcoded_caller_index = HardcodedCallerList::index(caller_addresses.caller);
        let caller_enum = CallerEnum::from(maybe_hardcoded_caller_index);

        // match_axes!(CM = caller_enum => {
        //     let locator = CM::get_locator(maybe_hardcoded_caller_index);
        // });

        // for (index, (token_data, token_delta)) in data_iter.zip(delta_iter).enumerate() {
        //     if *token_delta != TokenDelta::DEFAULT {
        //         let token_index = TM::TokenIndex::from(index);

        //         TokenSettler::<CustomCaller, TM> {
        //             token_index,
        //             locator: CustomCallerStub,
        //             token_data,
        //             token_delta,
        //             msg_transfers,
        //             caller_addresses,
        //         }
        //         .settle()?;
        //     }
        // }

        // TODO fix problem here

        //
        match_axes!(CM = caller_enum => {
            let locator = CM::get_locator(maybe_hardcoded_caller_index);

            for (index, (token_data, token_delta)) in
                data_iter.zip(delta_iter).enumerate()
            {
                if *token_delta != TokenDelta::DEFAULT {
                    let token_index = TM::TokenIndex::from(index);

                    // TODO fix error here
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_locator() {
        // This works for both cases. Unwrap does not cause problem
        let address = [1u8; 20];
        let maybe_hardcoded_caller_index = HardcodedCallerList::index(&address);
        let caller_enum = CallerEnum::from(maybe_hardcoded_caller_index);

        match_axes!(CM = caller_enum => {
            let locator = CM::get_locator(maybe_hardcoded_caller_index);

            println!("locator {:?}", locator);
        });
    }
}
