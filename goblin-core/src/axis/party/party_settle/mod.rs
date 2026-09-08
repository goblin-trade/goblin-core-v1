mod counterparties;
mod sender;

use crate::{
    axis::token::{token_marker::TokenMarker, token_reader::TokenDataTriple},
    goblin_error::GoblinError,
    input_processor::{CallerAddresses, MsgTransfers},
    settlement::global_delta::{GlobalDelta, TokenDelta},
};

pub trait PartySettle {
    fn settle<'a, TM>(
        global_delta: &'a GlobalDelta,
        token_data_triple: &TokenDataTriple<'a>,
        transfers: &MsgTransfers,
        caller_addresses: CallerAddresses<'a>,
    ) -> Result<(), GoblinError>
    where
        TM: TokenMarker,
        &'a TM::SenderDeltaList: IntoIterator<Item = &'a TokenDelta<TM>>;
}
