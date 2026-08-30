use goblin_macros::ConstDefault;

use crate::axis::token::token_quantity::TokenQuantity;

#[derive(Clone, Copy, ConstDefault)]
pub struct TokenData<TM: TokenQuantity> {
    pub address: TM::TokenAddress,
    pub decimals: TM::HardcodedDecimals,
}
