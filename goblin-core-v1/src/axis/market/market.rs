use crate::types::{Marker, Tuple};

/// Market axis
#[derive(Default, Clone, Copy, PartialEq)]
pub struct Market;

pub type Hardcoded = Marker<Market, 0>;
pub type Dynamic = Marker<Market, 1>;
