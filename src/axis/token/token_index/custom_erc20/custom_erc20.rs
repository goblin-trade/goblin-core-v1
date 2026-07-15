use crate::{
    axis::token::{
        token_index::{CustomERC20Index, CustomERC20List, TokenIndex},
        CustomERC20Stub,
    },
    goblin_error::GoblinError,
    hostio::erc20_hostio,
    types::Address,
};

impl TokenIndex for CustomERC20Index {
    const DISCRIMINATOR: u8 = 2;

    type DataList = CustomERC20List<'static>;

    type TokenAddress = Address;

    type HardcodedDecimals = CustomERC20Stub;
    type HostioDecimals = u8;

    type StoredDecimals = u8;
    type StoredPadding = [u8; 16 - size_of::<Self::StoredDecimals>()];

    fn get_address(&self, custom_erc20_list: CustomERC20List) -> Self::TokenAddress {
        custom_erc20_list[*self].address
    }

    fn get_hostio_decimals(
        &self,
        address: &Self::TokenAddress,
    ) -> Result<Self::HostioDecimals, GoblinError> {
        erc20_hostio::decimals(address)
    }
}
