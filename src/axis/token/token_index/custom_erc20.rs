use crate::{
    axis::token::{
        token_index::TokenIndex,
        token_marker::custom_erc20::{
            custom_erc20_index::CustomERC20Index, custom_erc20_list::CustomERC20List,
        },
        CustomERC20,
    },
    goblin_error::GoblinError,
    types::Address,
};

impl TokenIndex<CustomERC20> for CustomERC20Index {
    type TokenAddress = Address;

    fn get_address(
        &self,
        custom_erc20_list: CustomERC20List,
    ) -> Result<Self::TokenAddress, GoblinError> {
        custom_erc20_list.token_index_to_address(*self)
    }
}
