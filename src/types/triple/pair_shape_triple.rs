use crate::impl_triple_reader;
use crate::{
    token::{ERC20, ETH},
    types::{Triple, TripleReader},
};

impl_triple_reader!((ETH, ERC20), (ERC20, ETH), (ERC20, ERC20));

pub type PairShapeTriple<T0, T1, T2> =
    Triple<T0, T1, T2, ((ETH, ERC20), (ERC20, ETH), (ERC20, ERC20))>;
