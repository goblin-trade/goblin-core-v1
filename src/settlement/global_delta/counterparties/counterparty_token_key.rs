use crate::{axis::token::token_quantity::TokenQuantity, settlement::ConstZero, types::Address};

#[derive(PartialEq, Clone, Copy)]
pub struct CounterpartyTokenKey<T: TokenQuantity> {
    pub counterparty: Address,
    pub token_index: T::TokenIndex,
}

impl<T: TokenQuantity> ConstZero for CounterpartyTokenKey<T> {
    const ZEROED: Self = Self {
        counterparty: Address::ZEROED,
        token_index: T::TokenIndex::ZEROED,
    };
}
