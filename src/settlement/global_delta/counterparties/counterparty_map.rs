use crate::{
    axis::token::token_marker::TokenMarker,
    settlement::{
        global_delta::{CounterpartyTokenKey, DeltaAtomsPair},
        ConstZero,
    },
    types::FixedMap,
};

const MAX_COUNTERPARTY: usize = 16;

pub type CounterpartyMap<T> = FixedMap<CounterpartyTokenKey<T>, DeltaAtomsPair, MAX_COUNTERPARTY>;

impl<T: TokenMarker> ConstZero for CounterpartyMap<T> {
    const ZEROED: Self = Self {
        entries: [(CounterpartyTokenKey::ZEROED, DeltaAtomsPair::ZEROED); MAX_COUNTERPARTY],
        len: 0,
    };
}
