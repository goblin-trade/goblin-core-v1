/// Leg axis - the side of a trade
use crate::{define_axis, types::Tuple};

define_axis! {
    pub struct Leg;
    enum LegEnum {
        Base = 0,
        Quote = 1,
    }
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
