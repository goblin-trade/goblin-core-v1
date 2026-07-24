use crate::{
    axis::{leg::SamePair, token::token_quantity::TokenQuantity},
    settlement::{
        global_delta::{CounterpartyTokenKey, GlobalCounterparty},
        ConstZero,
    },
    types::FixedMap,
};

const MAX_COUNTERPARTY: usize = 16;

pub type CounterpartyMap<T> =
    FixedMap<CounterpartyTokenKey<T>, SamePair<GlobalCounterparty>, MAX_COUNTERPARTY>;

impl<T: TokenQuantity> ConstZero for CounterpartyMap<T> {
    const ZEROED: Self = Self {
        entries: [(
            CounterpartyTokenKey::ZEROED,
            SamePair::<GlobalCounterparty>::ZEROED,
        ); MAX_COUNTERPARTY],
        len: 0,
    };
}
