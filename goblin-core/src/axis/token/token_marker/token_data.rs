use goblin_macros::ConstDefault;

use crate::axis::token::TokenQuantity;

#[derive(Clone, Copy, ConstDefault)]
pub struct TokenData<TM: TokenQuantity> {
    pub address: TM::TokenAddress,
    pub decimals: TM::HardcodedDecimals,
}
