use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        token::token_marker::custom_erc20::custom_erc20_data::CustomERC20Data,
    },
    goblin_error::GoblinError,
    input_processor::Decodable,
    settlement::local_delta::Deposits,
};

/// Marker class for 'Token'. We have 3 variants- ETH, HardcodedERC20 and CustomERC20
pub trait TokenMarker: Clone + Copy + 'static {
    const DISCRIMINATOR: u8;

    /// Index to lookup token address
    type TokenIndex: Clone + Copy + Decodable;

    /// Token address
    type Address: Clone + Copy + Sized + Default;

    /// Data type representing pending deposit amount
    type Deposit: Clone + Copy + Default + Decodable;

    fn token_index_to_address(
        token_index: Self::TokenIndex,
        custom_erc20_list: &[CustomERC20Data],
    ) -> Result<Self::Address, GoblinError>;

    /// Save deposit amount in deposit store
    fn set_deposit<In>(deposits: &mut Deposits, deposit_amount: Self::Deposit)
    where
        In: LegMatcher;
}
