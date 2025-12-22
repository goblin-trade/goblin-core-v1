use crate::impl_tuple_reader;

use crate::{
    token::{ERC20, ETH},
    types::{Tuple, TupleReader},
};

impl_tuple_reader!(ETH, ERC20);

/// A generic container for ETH and ERC20 namespaced data
pub type TokenPair<T0, T1> = Tuple<T0, T1, (ETH, ERC20)>;
