use crate::impl_triple_reader;
use crate::{
    token::{CustomERC20, HardcodedERC20, ETH},
    types::{Triple, TripleReader},
};

impl_triple_reader!(ETH, HardcodedERC20, CustomERC20);

/// Accessor for the 3 token variants
pub type TokenTriple<T0, T1, T2> = Triple<T0, T1, T2, (ETH, HardcodedERC20, CustomERC20)>;
