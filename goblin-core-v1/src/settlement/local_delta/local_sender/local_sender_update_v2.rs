use crate::{
    axis::party::Sender,
    axis_helpers::TokenPair,
    settlement::local_delta::{LocalDeposits, LocalSender, LocalUpdate},
    types::StoreReader,
};

/// Wrapper struct for sender trade updates and deposits
#[derive(Clone, Copy)]
pub struct LocalSenderUpdateV2<TP: TokenPair> {
    pub sender: LocalSender,
    pub deposits: LocalDeposits<TP>,
}
