use crate::{
    axis::leg::SamePair,
    settlement::local_delta::LocalCounterparty,
    types::{Address, FixedMap},
};

pub const MAX_COUNTERPARTY: usize = 16;

pub type LocalCounterparties = FixedMap<Address, SamePair<LocalCounterparty>, MAX_COUNTERPARTY>;
