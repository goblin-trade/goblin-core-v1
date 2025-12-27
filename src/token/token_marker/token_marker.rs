use crate::{
    input_processor::Decodable, market::MarketVariant, quantities::DeltaAtoms, types::Address,
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
    type Deposit: Clone + Copy + Default + Decodable;
}

impl TokenMarker for ETH {
    const DISCRIMINATOR: u8 = 0;

    type TokenIndex<M: MarketVariant> = ();
    type Address = ();
    type Deposit = ();
}

impl TokenMarker for ERC20 {
    const DISCRIMINATOR: u8 = 1;

    type TokenIndex<M: MarketVariant> = M::TokenIndex;
    type Address = Address;
    type Deposit = DeltaAtoms;
}
