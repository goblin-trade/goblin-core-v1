use crate::{
    axis::token::{token_index::TokenData, token_marker::TokenMarker},
    input_processor::MsgTransfers,
    types::Address,
};

// why trait? We already have TokenDelta::sender(), a common API
//
// Do we want to expose the existence of lists or hide them?
// Better to expose. But try to improve the API
//
// Counterpoint
// Looping code is repeated in hardcoded and custom branches.
// Can we have cleaner API- just 3 calls to the 3 delta classes?
pub trait SenderDelta: TokenMarker {
    type IndexDataList;

    fn settle(
        &mut self,
        trader: &Address,
        token_index_and_data: (Self::TokenIndex, TokenData<Self>),
        msg_transfers: &MsgTransfers,
    );
}
