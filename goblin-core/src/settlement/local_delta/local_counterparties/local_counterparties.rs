use crate::{
    settlement::local_delta::LocalCounterparty,
    types::{Address, FixedMap},
};

pub const MAX_COUNTERPARTY: usize = 16;

pub type LocalCounterparties = FixedMap<Address, LocalCounterparty, MAX_COUNTERPARTY>;
