use crate::impl_tuple_reader;
use crate::quantities::DeltaAtoms;
use crate::token::TokenMarker;
use crate::{
    token::{ERC20, ETH},
    types::{Tuple, TupleReader},
};

impl_tuple_reader!(ETH, ERC20);

/// A generic container for ETH and ERC20 namespaced data
pub type TokenPair<T0, T1> = Tuple<T0, T1, (ETH, ERC20)>;

/// Track deposit amount for ETH and ERC20, for a given leg side
pub type DepositPair = TokenPair<<ETH as TokenMarker>::Deposit, <ERC20 as TokenMarker>::Deposit>;

impl DepositPair {
    pub fn zero() -> Self {
        Self::new((), DeltaAtoms::ZERO)
    }
}
