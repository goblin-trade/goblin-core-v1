use crate::{market::MarketVariant, quantities::DeltaAtoms, types::Address};

#[derive(Clone, Copy, Default)]
pub struct ETH;

#[derive(Clone, Copy, Default)]
pub struct ERC20;

pub trait TokenMarker: Clone + Copy {
    const DISCRIMINATOR: u8;

    // This is only used in dynamic markets, not hardcoded? We could remove M then
    type TokenIndex<M: MarketVariant>: Clone + Copy;

    type Address: Clone + Copy + Sized + Default;
    type Deposit: Clone + Copy + Default;
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
