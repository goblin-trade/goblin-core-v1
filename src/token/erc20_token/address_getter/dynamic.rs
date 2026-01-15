use crate::{
    goblin_error::GoblinError,
    token::{
        AddressGetter, CustomERC20, CustomERC20Store, DynamicIndex, HardcodedERC20, TokenIndex,
    },
    types::Address,
};

impl AddressGetter for DynamicIndex {
    fn address(self, custom_erc20_list: &[CustomERC20Store]) -> Result<Address, GoblinError> {
        match self {
            DynamicIndex::Hardcoded(token_index) => {
                TokenIndex::<HardcodedERC20>::address(token_index, custom_erc20_list)
            }
            DynamicIndex::Custom(token_index) => {
                TokenIndex::<CustomERC20>::address(token_index, custom_erc20_list)
            }
        }
    }
}
