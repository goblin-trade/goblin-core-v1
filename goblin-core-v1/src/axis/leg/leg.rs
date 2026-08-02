use crate::types::{Marker, Tuple};

/// Leg axis- the side of a trade
#[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Leg;

pub type Base = Marker<Leg, 0>;
pub type Quote = Marker<Leg, 1>;

#[derive(PartialEq, Clone, Copy)]
pub enum LegEnum {
    Base,
    Quote,
}

pub type Pair<T0, T1> = Tuple<T0, T1, Leg>;
pub type SamePair<T> = Pair<T, T>;

impl<T0> SamePair<T0> {
    pub fn into_pair<T1>(&self) -> SamePair<T1>
    where
        T0: Clone + Copy,
        T1: From<T0>,
    {
        Pair::new(self.0.into(), self.1.into())
    }
}
