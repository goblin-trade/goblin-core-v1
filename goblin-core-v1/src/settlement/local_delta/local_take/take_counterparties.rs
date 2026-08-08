use crate::{
    axis::leg::SamePair,
    settlement::local_delta::local_take::LocalCounterparty,
    types::{Address, FixedMap},
};

pub const MAX_COUNTERPARTY: usize = 16;

pub type TakeCounterparties = FixedMap<Address, SamePair<LocalCounterparty>, MAX_COUNTERPARTY>;
