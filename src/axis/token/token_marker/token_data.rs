use crate::axis::token::token_quantity::TokenQuantity;

#[derive(Clone, Copy)]
pub struct TokenData<T: TokenQuantity> {
    pub address: T::TokenAddress,
    pub decimals: T::HardcodedDecimals,
}
