use crate::{
    axis_helpers::TokenPair,
    settlement::local_delta::{LocalDelta, LocalDeposits},
};

pub struct LocalUpdate<'a, TP: TokenPair> {
    pub delta: &'a LocalDelta<'a>,
    pub deposits: LocalDeposits<TP>,
}
