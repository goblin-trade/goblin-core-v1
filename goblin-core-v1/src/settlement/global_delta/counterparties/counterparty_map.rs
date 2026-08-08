use crate::{
    settlement::global_delta::{CounterpartyTokenKey, GlobalCounterparty},
    types::FixedMap,
};

const MAX_COUNTERPARTY: usize = 16;

pub type CounterpartyMap<T> =
    FixedMap<CounterpartyTokenKey<T>, GlobalCounterparty, MAX_COUNTERPARTY>;
