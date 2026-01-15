use crate::{
    erc20,
    goblin_error::GoblinError,
    token::{ERC20Data, ERC20Marker, TokenIndex},
    types::Address,
};

#[derive(Clone, Copy, PartialEq)]
pub struct CustomERC20;

pub struct CustomERC20Store {
    pub address: Address,
}

impl ERC20Data for CustomERC20Store {
    fn address(&self) -> &Address {
        &self.address
    }

    fn decimals(&self) -> Result<u8, GoblinError> {
        erc20::decimals(&self.address)
    }
}

impl PartialEq for CustomERC20Store {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}

impl ERC20Marker for CustomERC20 {
    type Store = CustomERC20Store;

    fn get_token(
        token_index: TokenIndex<Self>,
        custom_erc20_list: &[CustomERC20Store],
    ) -> Result<&Self::Store, GoblinError> {
        custom_erc20_list
            .get(token_index.inner as usize)
            .ok_or(GoblinError::InvalidHardcodedTokenIndex)
    }
}
