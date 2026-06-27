use crate::settlement::{
    global_delta_v3::{CounterpartyMap, CounterpartyTriple},
    ConstZero,
};

impl ConstZero for CounterpartyTriple {
    const ZEROED: Self = Self::new(
        CounterpartyMap::ZEROED,
        CounterpartyMap::ZEROED,
        CounterpartyMap::ZEROED,
    );
}
