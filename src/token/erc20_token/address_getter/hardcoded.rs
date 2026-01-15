use crate::{
    goblin_error::GoblinError,
    token::{AddressGetter, CustomERC20Store, ERC20Marker, HardcodedERC20, TokenIndex},
    types::Address,
};

impl AddressGetter for TokenIndex<HardcodedERC20> {
    fn address(self, custom_erc20_list: &[CustomERC20Store]) -> Result<Address, GoblinError> {
        let store = HardcodedERC20::get_token(self, custom_erc20_list)?;
        Ok(store.address)
    }
}
