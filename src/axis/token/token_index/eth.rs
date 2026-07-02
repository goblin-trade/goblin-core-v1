use crate::{
    axis::token::{
        token_index::TokenIndex,
        token_marker::{custom_erc20::custom_erc20_list::CustomERC20List, eth::ETHStub},
        ETH,
    },
    goblin_error::GoblinError,
};

impl TokenIndex<ETH> for ETHStub {
    fn get_address(&self, _custom_erc20_list: CustomERC20List) -> Result<ETHStub, GoblinError> {
        Ok(ETHStub)
    }
}
