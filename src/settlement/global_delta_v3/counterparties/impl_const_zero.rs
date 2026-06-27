use crate::settlement::{
    global_delta_v3::{Counterparties, CounterpartyMap},
    ConstZero,
};

impl ConstZero for Counterparties {
    const ZEROED: Self = Self::new(
        CounterpartyMap::ZEROED,
        CounterpartyMap::ZEROED,
        CounterpartyMap::ZEROED,
    );
}
