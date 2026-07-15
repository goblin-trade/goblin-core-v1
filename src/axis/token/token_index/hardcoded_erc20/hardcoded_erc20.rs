use crate::{
    axis::token::{
        token_index::{
            CustomERC20List, HardcodedERC20Index, HardcodedTokens, TokenIndex,
            HARDCODED_ERC20_COUNT, HARDCODED_TOKENS,
        },
        HardcodedERC20Stub,
    },
    goblin_error::GoblinError,
    types::Address,
};

impl TokenIndex for HardcodedERC20Index {
    const DISCRIMINATOR: u8 = 1;

    type DataList = HardcodedTokens<HARDCODED_ERC20_COUNT>;

    type TokenAddress = Address;

    type HardcodedDecimals = u8;
    type HostioDecimals = HardcodedERC20Stub;

    type StoredDecimals = u8;
    type StoredPadding = [u8; 16 - size_of::<Self::StoredDecimals>()];

    fn get_address(&self, _custom_erc20_list: CustomERC20List) -> Self::TokenAddress {
        HARDCODED_TOKENS[*self].address
    }

    fn get_hostio_decimals(
        &self,
        _address: &Self::TokenAddress,
    ) -> Result<Self::HostioDecimals, GoblinError> {
        Ok(HardcodedERC20Stub)
    }
}
