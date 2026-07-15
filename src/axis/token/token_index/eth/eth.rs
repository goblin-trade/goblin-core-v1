use crate::{
    axis::token::{
        token_index::{CustomERC20List, TokenIndex},
        ETHStub,
    },
    goblin_error::GoblinError,
};

impl TokenIndex for ETHStub {
    const DISCRIMINATOR: u8 = 0;

    type DataList = ETHStub;

    type TokenAddress = ETHStub;

    type HardcodedDecimals = ETHStub;
    type HostioDecimals = ETHStub;

    // Store 18 decimals in ETHStore for symmetry?
    type StoredDecimals = ETHStub;
    type StoredPadding = [u8; 16 - size_of::<Self::StoredDecimals>()];

    fn get_address(
        &self,
        _custom_erc20_list: CustomERC20List,
    ) -> Result<Self::TokenAddress, GoblinError> {
        Ok(ETHStub)
    }

    fn get_hostio_decimals(
        &self,
        _address: &Self::TokenAddress,
    ) -> Result<Self::HostioDecimals, GoblinError> {
        Ok(ETHStub)
    }
}
