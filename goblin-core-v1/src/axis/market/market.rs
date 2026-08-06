use crate::types::Marker;

/// Market axis
#[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Market;

pub type Hardcoded = Marker<Market, 0>;
pub type Dynamic = Marker<Market, 1>;
