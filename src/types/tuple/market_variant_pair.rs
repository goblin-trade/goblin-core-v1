use crate::impl_tuple_reader;
use crate::{
    token::{DynamicIndex, HardcodedIndex},
    types::{Tuple, TupleReader},
};

impl_tuple_reader!(HardcodedIndex, DynamicIndex);

/// A generic container for hardcoded and dynamic market namespaced data
pub type MarketVariantPair<T0, T1> = Tuple<T0, T1, (HardcodedIndex, DynamicIndex)>;
