use crate::{axis::token::token_deltas::TokenDeltas, settlement::ConstZero, types::Address};

#[derive(PartialEq, Clone, Copy)]
pub struct CounterpartyTokenKey<T: TokenDeltas> {
    pub counterparty: Address,
    pub token_index: T::TokenIndex,
}

impl<T: TokenDeltas> ConstZero for CounterpartyTokenKey<T> {
    const ZEROED: Self = Self {
        counterparty: Address::ZEROED,
        token_index: T::TokenIndex::ZEROED,
    };
}
