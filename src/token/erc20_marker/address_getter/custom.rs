use crate::{
    goblin_error::GoblinError,
    token::{AddressMapper, CustomERC20, CustomERC20Data, ERC20Marker, TokenIndex},
    types::Address,
};

impl AddressMapper for TokenIndex<CustomERC20> {
    fn address(self, custom_erc20_list: &[CustomERC20Data]) -> Result<Address, GoblinError> {
        let store = CustomERC20::get_data(self, custom_erc20_list)?;
        Ok(store.address)
    }
}
