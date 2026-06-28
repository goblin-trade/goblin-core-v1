use crate::settlement::{
    global_delta::{CounterpartyMap, CounterpartyTriple},
    ConstZero,
};

impl ConstZero for CounterpartyTriple {
    const ZEROED: Self = Self::new(
        CounterpartyMap::ZEROED,
        CounterpartyMap::ZEROED,
        CounterpartyMap::ZEROED,
    );
}
