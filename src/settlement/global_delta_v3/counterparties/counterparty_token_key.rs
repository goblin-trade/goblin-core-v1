use crate::{axis::token::token_marker::TokenMarker, settlement::ConstZero, types::Address};

#[derive(PartialEq, Clone, Copy)]
pub struct CounterpartyTokenKey<T: TokenMarker> {
    pub counterparty: Address,
    pub token_index: T::TokenIndex,
}

impl<T: TokenMarker> ConstZero for CounterpartyTokenKey<T> {
    const ZEROED: Self = Self {
        counterparty: Address::ZEROED,
        token_index: T::TokenIndex::ZEROED,
    };
}
