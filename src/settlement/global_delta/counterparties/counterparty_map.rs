use crate::{
    axis::token::token_quantity::TokenQuantity,
    quantities::UnsidedDeltaAtoms,
    settlement::{global_delta::CounterpartyTokenKey, ConstZero},
    types::FixedMap,
};

const MAX_COUNTERPARTY: usize = 16;

pub type CounterpartyMap<T> =
    FixedMap<CounterpartyTokenKey<T>, UnsidedDeltaAtoms, MAX_COUNTERPARTY>;

impl<T: TokenQuantity> ConstZero for CounterpartyMap<T> {
    const ZEROED: Self = Self {
        entries: [(CounterpartyTokenKey::ZEROED, UnsidedDeltaAtoms::ZEROED); MAX_COUNTERPARTY],
        len: 0,
    };
}
