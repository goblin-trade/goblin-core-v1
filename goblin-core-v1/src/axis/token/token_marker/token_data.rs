use goblin_macros::ConstZero;

use crate::axis::token::token_quantity::TokenQuantity;

#[derive(Clone, Copy, ConstZero)]
pub struct TokenData<T: TokenQuantity> {
    pub address: T::TokenAddress,
    pub decimals: T::HardcodedDecimals,
}
