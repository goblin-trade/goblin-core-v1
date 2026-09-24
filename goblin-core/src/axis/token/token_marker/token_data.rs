use goblin_macros::ConstDefault;

use crate::axis::token::TokenQuantity;

/// Marked `repr(C)` so the zero-copy list decode in
/// [`CustomERC20List`](crate::axis::token::token_list::custom_erc20::CustomERC20List)
/// can reinterpret contiguous calldata bytes as `[TokenData<CustomERC20>]`.
#[derive(Clone, Copy, ConstDefault)]
#[repr(C)]
pub struct TokenData<TM: TokenQuantity> {
    pub address: TM::TokenAddress,
    pub decimals: TM::HardcodedDecimals,
}
