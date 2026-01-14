use crate::{
    goblin_error::GoblinError,
    token::{CustomERC20Store, TokenIndex},
    types::Address,
};
/// Trait for ERC20 tokens
pub trait ERC20Marker: Sized {
    type Store;

    fn get_token(
        token_index: TokenIndex<Self>,
        custom_erc20_list: &[CustomERC20Store],
    ) -> Result<&Self::Store, GoblinError>;

    /// Token address
    fn address(store: &Self::Store) -> &Address;

    /// Token decimals
    fn decimals(store: &Self::Store) -> Result<u8, GoblinError>;
}
