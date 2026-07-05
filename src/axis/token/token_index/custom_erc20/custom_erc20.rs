use crate::{
    axis::token::token_index::{CustomERC20Index, CustomERC20List, TokenIndex},
    goblin_error::GoblinError,
    types::Address,
};

impl TokenIndex for CustomERC20Index {
    const DISCRIMINATOR: u8 = 2;
    type TokenAddress = Address;

    type StoredDecimals = u8;
    type StoredPadding = [u8; 16 - size_of::<Self::StoredDecimals>()];

    fn get_address(
        &self,
        custom_erc20_list: CustomERC20List,
    ) -> Result<Self::TokenAddress, GoblinError> {
        custom_erc20_list.token_index_to_address(*self)
    }
}
