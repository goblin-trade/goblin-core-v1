use crate::quantities::DeltaAtoms;
use crate::token::{TokenMarker, ERC20, ETH};
use crate::types::TokenPair;

/// Track deposit amount for ETH and ERC20, for a given leg side
pub type DepositForSide = TokenPair<<ETH as TokenMarker>::Deposit, <ERC20 as TokenMarker>::Deposit>;

impl DepositForSide {
    pub const fn zero() -> Self {
        Self::new((), DeltaAtoms::ZERO)
    }
}
