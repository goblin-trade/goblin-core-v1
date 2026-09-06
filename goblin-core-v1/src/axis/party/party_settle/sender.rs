use core::iter::Zip;

use crate::{
    axis::{
        party::{PartySettle, Sender},
        token::{token_marker::TokenMarker, token_reader::TokenDataTriple},
        CallerData, CallerMarker, HardcodedCaller, TokenList,
    },
    for_axes,
    goblin_error::GoblinError,
    input_processor::MsgTransfers,
    settlement::{
        global_delta::{GlobalDelta, TokenDelta},
        ConstDefault, TokenSettler,
    },
    types::{Address, StoreReader},
};

impl PartySettle for Sender {
    fn settle<'a, TM>(
        caller: &Address,
        recipient: &Address,
        global_delta: &'a GlobalDelta,
        token_data_triple: &TokenDataTriple<'a>,
        msg_transfers: &MsgTransfers,
    ) -> Result<(), GoblinError>
    where
        TM: TokenMarker,
        &'a TM::SenderDeltaList: IntoIterator<Item = &'a TokenDelta<TM>>,
    {
        let sender_delta = Self::get_leg(global_delta);

        // problem- data and delta lists should not implement copy
        let data_list_iter = TM::get_lifetimed(&token_data_triple).into_iter();
        let sender_delta_list_iter = TM::get_leg(sender_delta).into_iter();

        let iter = data_list_iter.zip(sender_delta_list_iter);

        // i am back to CallerIndexEnum
        // this enum is equivalent to Option<HardcodedCallerIndex>

        let locator = HardcodedCaller::get_locator(caller);

        // if let Some(locator) = HardcodedCaller::get_locator(caller) {
        //     let caller_data = CallerData::<CM> {
        //         address: caller,
        //         locator,
        //     };

        //     for (index, (token_data, token_delta)) in
        //         data_list_iter.zip(sender_delta_list_iter).enumerate()
        //     {
        //         if *token_delta != TokenDelta::DEFAULT {
        //             let token_index = TM::TokenIndex::from(index);

        //             TokenSettler {
        //                 token_index,
        //                 token_data,
        //                 token_delta,
        //                 caller_data,
        //                 msg_transfers,
        //                 recipient,
        //             }
        //             .settle()?;
        //         }
        //     }
        // }

        // for_axes gives copy clone issue
        // for_axes!(CM => {
        //     if let Some(locator) = CM::get_locator(caller) {
        //         let caller_data = CallerData::<CM> {
        //             address: caller,
        //             locator
        //         };

        //         for (index, (token_data, token_delta)) in
        //             data_list_iter.zip(sender_delta_list_iter).enumerate()
        //         {
        //             if *token_delta != TokenDelta::DEFAULT {
        //                 let token_index = TM::TokenIndex::from(index);

        //                 TokenSettler {
        //                     token_index,
        //                     token_data,
        //                     token_delta,
        //                     caller_data,
        //                     msg_transfers,
        //                     recipient,
        //                 }
        //                 .settle()?;
        //             }
        //         }
        //     }
        // });

        Ok(())
    }
}

fn settle_inner<'a, CM, TM>(
    caller_data: CallerData<CM>,
    recipient: &Address,
    msg_transfers: &MsgTransfers,
    iter: Zip<
        <<TM as TokenList>::DataList<'_> as IntoIterator>::IntoIter,
        <&'a <TM as TokenList>::SenderDeltaList as IntoIterator>::IntoIter,
    >,
    // data_list_iter: <<TM as TokenList>::DataList<'_> as IntoIterator>::IntoIter,
    // sender_delta_list_iter: <&'a <TM as TokenList>::SenderDeltaList as IntoIterator>::IntoIter,
) -> Result<(), GoblinError>
where
    CM: CallerMarker,
    TM: TokenMarker,
    &'a TM::SenderDeltaList: IntoIterator<Item = &'a TokenDelta<TM>>,
{
    for (index, (token_data, token_delta)) in iter.enumerate() {
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
    Ok(())
}
