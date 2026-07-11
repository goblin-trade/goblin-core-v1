use crate::{
    axis::token::token_index::{
        CustomERC20List, HardcodedERC20Index, TokenIndex, HARDCODED_TOKENS,
    },
    goblin_error::GoblinError,
    types::Address,
};

impl TokenIndex for HardcodedERC20Index {
    const DISCRIMINATOR: u8 = 1;
    type TokenAddress = Address;

    type HardcodedDecimals = u8;

    type StoredDecimals = u8;
    type StoredPadding = [u8; 16 - size_of::<Self::StoredDecimals>()];

    fn get_address(
        &self,
        _custom_erc20_list: CustomERC20List,
    ) -> Result<Self::TokenAddress, GoblinError> {
        HARDCODED_TOKENS.get(*self).map(|data| data.address)
    }

    fn get_decimals(
        &self,
        _address: &Self::TokenAddress,
    ) -> Result<Self::StoredDecimals, GoblinError> {
        HARDCODED_TOKENS.get(*self).map(|data| data.decimals)
    }
}
