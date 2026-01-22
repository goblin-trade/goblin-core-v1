use crate::{
    goblin_error::GoblinError,
    quantities::DeltaAtoms,
    settlement::local_delta::Deposits,
    token::CustomERC20Data,
    types::{Base, LegMatcher, Quote, TupleReader},
};

/// Marker class for 'Token'. We have 2 tokens
///
/// 1. ETH
/// 2. ERC20- this has sub variants hardcoded and custom, covered by ERC20Marker
pub trait TokenMarker: Clone + Copy {
    const DISCRIMINATOR: u8;

    // type TupleMarker: TupleMarker;

    /// Index to lookup token address
    type TokenIndex: Clone + Copy;

    /// Token address
    type Address: Clone + Copy + Sized + Default;

    /// Data type representing pending deposit amount
    type Deposit: Clone + Copy + Default;

    /// Map the token index to address
    ///
    /// This function has 2 variations for TokenMarker and MarketVariant traits
    ///
    /// 1. TokenMarker (ETH / ERC20): Maps ETH to (). If token is ERC20 then
    /// calls MarketVariant::token_index_to_address()
    ///
    /// 2. MarketVariant (Hardcoded / Dynamic): Reads token address from
    /// hardcoded or custom token list
    fn token_index_to_address(
        token_index: Self::TokenIndex,
        custom_erc20_list: &[CustomERC20Data],
    ) -> Result<Self::Address, GoblinError>;

    /// Save deposit amount in deposit store
    fn set_deposit<In>(deposits: &mut Deposits, deposit_amount: Self::Deposit)
    where
        In: LegMatcher + TupleReader<DeltaAtoms, DeltaAtoms, (Base, Quote), Result = DeltaAtoms>;
}
