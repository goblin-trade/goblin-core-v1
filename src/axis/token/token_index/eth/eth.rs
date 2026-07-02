use crate::{
    axis::token::token_index::{eth::ETHStub, CustomERC20List, TokenIndex},
    goblin_error::GoblinError,
};

impl TokenIndex for ETHStub {
    type TokenAddress = ETHStub;

    fn get_address(
        &self,
        _custom_erc20_list: CustomERC20List,
    ) -> Result<Self::TokenAddress, GoblinError> {
        Ok(ETHStub)
    }
}
