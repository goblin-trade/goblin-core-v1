use crate::{
    axis::token::token_index::{eth::ETHStub, CustomERC20List, TokenIndex},
    goblin_error::GoblinError,
};

impl TokenIndex for ETHStub {
    const DISCRIMINATOR: u8 = 0;
    type TokenAddress = ETHStub;

    type StoredDecimals = ETHStub;
    type StoredPadding = [u8; 16 - size_of::<Self::StoredDecimals>()];

    fn get_address(
        &self,
        _custom_erc20_list: CustomERC20List,
    ) -> Result<Self::TokenAddress, GoblinError> {
        Ok(ETHStub)
    }
}
