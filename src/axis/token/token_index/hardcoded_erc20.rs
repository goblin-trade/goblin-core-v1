use crate::{
    axis::token::{
        token_index::TokenIndex,
        token_marker::{
            custom_erc20::custom_erc20_list::CustomERC20List,
            hardcoded_erc20::{HardcodedERC20Index, HARDCODED_TOKENS},
        },
        HardcodedERC20,
    },
    goblin_error::GoblinError,
    types::Address,
};

impl TokenIndex<HardcodedERC20> for HardcodedERC20Index {
    fn get_address(&self, _custom_erc20_list: CustomERC20List) -> Result<Address, GoblinError> {
        HARDCODED_TOKENS.get(*self).map(|data| data.address)
    }
}
