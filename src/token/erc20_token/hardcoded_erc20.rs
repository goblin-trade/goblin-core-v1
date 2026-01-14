use crate::{
    goblin_error::GoblinError,
    token::{CustomERC20Store, ERC20Marker, TokenIndex, HARDCODED_TOKENS},
    types::Address,
};

#[derive(Clone, Copy, PartialEq)]
pub struct HardcodedERC20;

pub struct HardcodedERC20Store {
    pub address: Address,
    pub decimals: u8,
}

impl ERC20Marker for HardcodedERC20 {
    type Store = HardcodedERC20Store;

    fn get_token(
        token_index: TokenIndex<Self>,
        _custom_erc20_list: &[CustomERC20Store],
    ) -> Result<&Self::Store, GoblinError> {
        // Safe because hardcoded index is validated
        let store = unsafe { HARDCODED_TOKENS.get_unchecked(token_index.inner as usize) };
        Ok(store)
    }

    fn address(store: &Self::Store) -> &Address {
        &store.address
    }

    fn decimals(store: &Self::Store) -> Result<u8, GoblinError> {
        Ok(store.decimals)
    }
}
