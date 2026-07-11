use crate::axis::token::{token_index::TokenIndex, token_marker::TokenMarker};

#[derive(Clone, Copy)]
pub struct TokenData<T: TokenIndex> {
    pub address: T::TokenAddress,
    pub decimals: T::HardcodedDecimals,
}
