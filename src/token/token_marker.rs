use crate::{quantities::DeltaAtoms, types::Address};

#[derive(Clone, Copy)]
pub struct ETH;

#[derive(Clone, Copy)]
pub struct ERC20;

pub trait TokenMarker: Clone + Copy {
    type MarkerAddress;
    type Deposit;
}

impl TokenMarker for ETH {
    type MarkerAddress = ();
    type Deposit = ();
}

impl TokenMarker for ERC20 {
    type MarkerAddress = Address;
    type Deposit = DeltaAtoms;
}
