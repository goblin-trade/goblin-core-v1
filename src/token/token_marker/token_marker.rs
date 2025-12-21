use crate::{markets::MarketVariant, quantities::DeltaAtoms, types::Address};

#[derive(Clone, Copy)]
pub struct ETH;

#[derive(Clone, Copy)]
pub struct ERC20;

pub trait TokenMarker: Clone + Copy {
    const DISCRIMINATOR: u8;
    const ADDRESS_SIZE: usize = 1;
    // const ADDRESS_SIZE: usize = core::mem::size_of::<Self::Address>();

    type TokenIndex<M: MarketVariant>: Clone + Copy;
    type Address: Clone + Copy + Sized;
    type Deposit: Clone + Copy;
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
