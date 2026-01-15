use crate::{
    goblin_error::GoblinError,
    token::{
        AddressMapper, CustomERC20, CustomERC20Data, DynamicIndex, HardcodedERC20, ERC20Index,
    },
    types::Address,
};

impl AddressMapper for DynamicIndex {
    fn address(self, custom_erc20_list: &[CustomERC20Data]) -> Result<Address, GoblinError> {
        match self {
            DynamicIndex::Hardcoded(token_index) => {
                ERC20Index::<HardcodedERC20>::address(token_index, custom_erc20_list)
            }
            DynamicIndex::Custom(token_index) => {
                ERC20Index::<CustomERC20>::address(token_index, custom_erc20_list)
            }
        }
    }
}
