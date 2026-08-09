use goblin_macros::ConstDefault;

use crate::axis::token::token_quantity::TokenQuantity;

#[derive(Clone, Copy, ConstDefault)]
pub struct TokenData<T: TokenQuantity> {
    pub address: T::TokenAddress,
    pub decimals: T::HardcodedDecimals,
}
