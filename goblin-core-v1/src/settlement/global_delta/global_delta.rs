use crate::{
    axis::party::Party,
    settlement::global_delta::{CounterpartyTriple, GlobalSender},
    types::Tuple,
};

pub type GlobalDelta = Tuple<GlobalSender, CounterpartyTriple, Party>;
