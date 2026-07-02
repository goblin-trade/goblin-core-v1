use crate::{
    axis::token::token_index::{
        CustomERC20List, HardcodedERC20Index, TokenIndex, HARDCODED_TOKENS,
    },
    goblin_error::GoblinError,
    types::Address,
};

impl TokenIndex for HardcodedERC20Index {
    type TokenAddress = Address;

    fn get_address(
        &self,
        _custom_erc20_list: CustomERC20List,
    ) -> Result<Self::TokenAddress, GoblinError> {
        HARDCODED_TOKENS.get(*self).map(|data| data.address)
    }
}
