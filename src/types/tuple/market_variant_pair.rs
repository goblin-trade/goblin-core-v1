use crate::impl_tuple_reader;
use crate::market::{Dynamic, Hardcoded};
use crate::types::{Tuple, TupleReader};

impl_tuple_reader!(Hardcoded, Dynamic);

/// A generic container for hardcoded and dynamic market namespaced data
pub type MarketVariantPair<T0, T1> = Tuple<T0, T1, (Hardcoded, Dynamic)>;
