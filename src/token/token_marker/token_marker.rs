use crate::{
    goblin_error::GoblinError, input_processor::Decodable, market::MarketVariant,
    quantities::DeltaAtoms, token::CustomToken, types::Address,
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

    fn token_index_to_address_outer<M: MarketVariant>(
        token_index: Self::TokenIndex<M>,
        custom_erc20_list: &[CustomToken],
    ) -> Result<Self::Address, GoblinError>;
}

impl TokenMarker for ETH {
    const DISCRIMINATOR: u8 = 0;

    type TokenIndex<M: MarketVariant> = ();
    type Address = ();
    type Deposit = ();

    fn token_index_to_address_outer<M: MarketVariant>(
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

    fn token_index_to_address_outer<M: MarketVariant>(
        token_index: Self::TokenIndex<M>,
        custom_erc20_list: &[CustomToken],
    ) -> Result<Self::Address, GoblinError> {
        M::token_index_to_address_inner(token_index, custom_erc20_list)
    }
}
