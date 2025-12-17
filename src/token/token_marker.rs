use crate::{quantities::DeltaAtoms, types::Address};

#[derive(Clone, Copy)]
pub struct ETH;

#[derive(Clone, Copy)]
pub struct ERC20;

pub trait TokenMarker: Clone + Copy {
    const DISCRIMINATOR: u8;

    type Address;
    type Deposit;
}

impl TokenMarker for ETH {
    const DISCRIMINATOR: u8 = 0;

    type Address = ();
    type Deposit = ();
}

impl TokenMarker for ERC20 {
    const DISCRIMINATOR: u8 = 1;

    type Address = Address;
    type Deposit = DeltaAtoms;
}
