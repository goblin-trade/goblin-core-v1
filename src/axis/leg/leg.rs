use crate::types::{Marker, Tuple};

/// Leg axis- the side of a trade
#[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Leg;

pub type Base = Marker<Leg, 0>;
pub type Quote = Marker<Leg, 1>;

pub type Pair<T0, T1> = Tuple<T0, T1, Leg>;

pub enum LegEnum {
    Base,
    Quote,
}
