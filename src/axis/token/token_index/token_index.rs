use crate::{
    axis::token::token_index::CustomERC20List, goblin_error::GoblinError,
    input_processor::Decodable, settlement::ConstZero,
};

pub trait TokenIndex: Clone + Copy + Decodable + ConstZero + PartialEq {
    const DISCRIMINATOR: u8;

    type TokenAddress: Clone + Copy + Sized + Default;

    type HardcodedDecimals: Clone + Copy;

    /// Decimals stored in `Store`
    /// Decimals are stored as u8 for ERC20 tokens but not for ETH
    type StoredDecimals: Clone + Copy + Into<u8>;

    /// Padding to pad `Store` to 32 bytes
    /// ERC20 store has less padding to accomodate `decimals: u8`
    type StoredPadding: Clone + Copy;

    fn get_address(
        &self,
        custom_erc20_list: CustomERC20List,
    ) -> Result<Self::TokenAddress, GoblinError>;

    // problem-
    //
    // - CustomERC20: decimals is derived from address
    // - HardcodedERC20: derived from token index (Self)
    //
    // But we don't want to pre-emptively read decimals from Hostio
    // for CustomERC20. First check if stored in store
    fn get_decimals(
        &self,
        address: &Self::TokenAddress,
    ) -> Result<Self::StoredDecimals, GoblinError>;
}
