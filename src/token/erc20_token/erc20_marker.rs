use crate::{
    goblin_error::GoblinError,
    token::{CustomERC20Store, ERC20Data, TokenIndex},
};
/// Trait for ERC20 tokens
pub trait ERC20Marker: Sized {
    type Store;

    fn get_token(
        token_index: TokenIndex<Self>,
        custom_erc20_list: &[CustomERC20Store],
    ) -> Result<&Self::Store, GoblinError>;
}
