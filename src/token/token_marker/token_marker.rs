use crate::{markets::MarketVariant, quantities::DeltaAtoms, types::Address};

#[derive(Clone, Copy)]
pub struct ETH;

#[derive(Clone, Copy)]
pub struct ERC20;

pub trait TokenMarker: Clone + Copy {
    const DISCRIMINATOR: u8;

    type TokenIndex<M: MarketVariant>;
    type Address;
    type Deposit;
}

impl TokenMarker for ETH {
    const DISCRIMINATOR: u8 = 0;

    type TokenIndex<M: MarketVariant> = ();
    type Address = ();
    type Deposit = ();
}

impl TokenMarker for ERC20 {
    const DISCRIMINATOR: u8 = 1;

    type TokenIndex<M: MarketVariant> = M;
    type Address = Address;
    type Deposit = DeltaAtoms;
}
