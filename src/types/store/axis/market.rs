use crate::types::{Marker, Tuple};

/// Market axis
#[derive(Default, Clone, Copy, PartialEq)]
pub struct Market;

pub type Hardcoded = Marker<Market, 0>;
pub type Dynamic = Marker<Market, 1>;

pub type MarketVariantPair<T0, T1> = Tuple<T0, T1, Market>;
