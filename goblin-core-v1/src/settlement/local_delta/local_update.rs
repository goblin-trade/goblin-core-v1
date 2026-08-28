use crate::{
    axis_helpers::TokenPair,
    settlement::local_delta::{LocalDelta, LocalDeposits},
};

pub struct LocalUpdate<'a, TP: TokenPair> {
    pub local_delta: &'a LocalDelta<'a>,
    pub local_deposits: LocalDeposits<TP>,
}
