use crate::impl_tuple_reader;
use crate::{
    token::{CustomToken, HardcodedToken},
    types::{Tuple, TupleReader},
};

impl_tuple_reader!(HardcodedToken, CustomToken);

/// A generic container for HardcodedToken and CustomToken namespaced data
pub type ERC20Pair<T0, T1> = Tuple<T0, T1, (HardcodedToken, CustomToken)>;
