use crate::{
    axis::token::token_index::{CustomERC20Index, CustomERC20List, TokenIndex},
    goblin_error::GoblinError,
    types::Address,
};

impl TokenIndex for CustomERC20Index {
    type TokenAddress = Address;

    fn get_address(
        &self,
        custom_erc20_list: CustomERC20List,
    ) -> Result<Self::TokenAddress, GoblinError> {
        custom_erc20_list.token_index_to_address(*self)
    }
}
