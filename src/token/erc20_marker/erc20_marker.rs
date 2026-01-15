use crate::{
    goblin_error::GoblinError,
    token::{CustomERC20Data, ERC20Data, TokenIndex},
};

/// Marker trait for ERC20 tokens
///
/// It has 2 variants
/// 1. Hardcoded- Token data is hardcoded
/// 2. Custom- Token data is read at runtime
///
pub trait ERC20Marker: Sized {
    type Data: ERC20Data;

    fn get_data(
        token_index: TokenIndex<Self>,
        custom_erc20_list: &[CustomERC20Data],
    ) -> Result<&Self::Data, GoblinError>;
}
