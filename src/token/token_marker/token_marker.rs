use crate::{
    goblin_error::GoblinError, market::MarketVariant, quantities::DeltaAtoms, token::CustomToken,
    types::Address,
};

#[derive(Clone, Copy, Default)]
pub struct ETH;

#[derive(Clone, Copy, Default)]
pub struct ERC20;

pub trait TokenMarker: Clone + Copy {
    const DISCRIMINATOR: u8;

    /// Index to lookup token address
    type TokenIndex<M: MarketVariant>: Clone + Copy;

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
    fn token_index_to_address<M: MarketVariant>(
        token_index: Self::TokenIndex<M>,
        custom_erc20_list: &[CustomToken],
    ) -> Result<Self::Address, GoblinError>;
}

impl TokenMarker for ETH {
    const DISCRIMINATOR: u8 = 0;

    type TokenIndex<M: MarketVariant> = ();
    type Address = ();
    type Deposit = ();

    fn token_index_to_address<M: MarketVariant>(
        _token_index: Self::TokenIndex<M>,
        _custom_erc20_list: &[CustomToken],
    ) -> Result<Self::Address, GoblinError> {
        Ok(())
    }
}

impl TokenMarker for ERC20 {
    const DISCRIMINATOR: u8 = 1;

    type TokenIndex<M: MarketVariant> = M::TokenIndex;
    type Address = Address;
    type Deposit = DeltaAtoms;

    fn token_index_to_address<M: MarketVariant>(
        token_index: Self::TokenIndex<M>,
        custom_erc20_list: &[CustomToken],
    ) -> Result<Self::Address, GoblinError> {
        M::token_index_to_address(token_index, custom_erc20_list)
    }
}
