use crate::types::{Leg, Market, Tuple};

pub type Pair<T0, T1> = Tuple<T0, T1, Leg>;

pub type MarketVariantPair<T0, T1> = Tuple<T0, T1, Market>;
