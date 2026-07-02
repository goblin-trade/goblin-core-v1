use crate::{
    axis::token::{
        token_index::TokenIndex,
        token_marker::{custom_erc20::custom_erc20_list::CustomERC20List, eth::ETHStub},
        ETH,
    },
    goblin_error::GoblinError,
};

impl TokenIndex<ETH> for ETHStub {
    type TokenAddress = ETHStub;

    fn get_address(
        &self,
        _custom_erc20_list: CustomERC20List,
    ) -> Result<Self::TokenAddress, GoblinError> {
        Ok(ETHStub)
    }
}
