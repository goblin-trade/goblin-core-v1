use crate::{
    axis_helpers::TokenPair,
    settlement::local_delta::{LocalDelta, LocalDeposits},
};

/// Unified struct for local delta and deposits
#[derive(Clone, Copy)]
pub struct LocalUpdate<'a, TP: TokenPair> {
    pub delta: &'a LocalDelta<'a>,
    pub deposits: LocalDeposits<TP>,
}
