use crate::impl_tuple_reader;
use crate::{
    token::{CustomERC20, HardcodedERC20},
    types::{Tuple, TupleReader},
};

impl_tuple_reader!(HardcodedERC20, CustomERC20);

/// A generic container for HardcodedToken and CustomToken namespaced data
pub type ERC20Pair<T0, T1> = Tuple<T0, T1, (HardcodedERC20, CustomERC20)>;
