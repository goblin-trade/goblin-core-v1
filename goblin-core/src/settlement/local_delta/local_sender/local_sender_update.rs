use crate::{
    axis_helpers::TokenPair,
    settlement::local_delta::{LocalDeposits, LocalSender},
};

/// Wrapper struct for sender trade updates and deposits
#[derive(Clone, Copy)]
pub struct LocalSenderUpdate<TP: TokenPair> {
    pub sender: LocalSender,
    pub deposits: LocalDeposits<TP>,
}
