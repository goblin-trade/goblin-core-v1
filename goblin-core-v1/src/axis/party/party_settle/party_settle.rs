use crate::{
    axis::token::{token_marker::TokenMarker, token_reader::TokenDataTriple},
    goblin_error::GoblinError,
    input_processor::MsgTransfers,
    settlement::global_delta::{GlobalDelta, TokenDelta},
    types::Address,
};

pub trait PartySettle {
    fn settle<'a, TM>(
        global_delta: &'a GlobalDelta,
        token_data_triple: &TokenDataTriple<'a>,
        trader: &Address,
        transfers: &MsgTransfers,
    ) -> Result<(), GoblinError>
    where
        TM: TokenMarker,
        &'a TM::SenderDeltaList: IntoIterator<Item = &'a TokenDelta<TM>>;
}
