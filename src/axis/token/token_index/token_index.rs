use core::ops::Index;

use crate::{
    axis::token::token_index::{CustomERC20List, TokenData},
    goblin_error::GoblinError,
    input_processor::Decodable,
    settlement::ConstZero,
};

pub trait TokenIndex: Clone + Copy + Decodable + ConstZero + PartialEq {
    type TokenAddress: Clone + Copy + Sized + Default;

    /// Decimals hardcoded in the smart contract
    type HardcodedDecimals: Clone + Copy;

    /// Decimals read from ERC20 hostio
    /// Only present for custom tokens
    type HostioDecimals: Clone + Copy;

    /// Decimals stored in `Store`
    /// Decimals are stored as u8 for ERC20 tokens but not for ETH
    ///
    /// TODO fix spagetti code
    /// We need a unified function
    /// StoredDecimals = f(HardcodedDecimals, HostioDecimals)
    type StoredDecimals: Clone
        + Copy
        + Into<u8>
        + TryFrom<Self::HardcodedDecimals>
        + TryFrom<Self::HostioDecimals>;

    /// Padding to pad `Store` to 32 bytes
    /// ERC20 store has less padding to accomodate `decimals: u8`
    type StoredPadding: Clone + Copy;

    // fn get_token_data(&self, custom_erc20_list: CustomERC20List) -> TokenData<Self> {

    // }

    // TODO replace custom function
    // Use Index trait
    //
    // TODO this is a safe function now as TokenIndex is bounds checked
    fn get_address(&self, custom_erc20_list: CustomERC20List) -> Self::TokenAddress;

    fn get_hostio_decimals(
        &self,
        address: &Self::TokenAddress,
    ) -> Result<Self::HostioDecimals, GoblinError>;
}
