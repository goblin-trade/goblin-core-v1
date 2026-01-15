use crate::{
    goblin_error::GoblinError,
    token::{AddressGetter, CustomERC20, CustomERC20Store, ERC20Marker, TokenIndex},
    types::Address,
};

impl AddressGetter for TokenIndex<CustomERC20> {
    fn address(self, custom_erc20_list: &[CustomERC20Store]) -> Result<Address, GoblinError> {
        let store = CustomERC20::get_token(self, custom_erc20_list)?;
        Ok(store.address)
    }
}
