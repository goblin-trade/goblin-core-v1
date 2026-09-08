use crate::{axis::leg::LegMatcher, quantities::UnsideQuantity, types::Tuple};

impl<S0, S1, T0, T1, K> UnsideQuantity<S0> for Tuple<T0, T1, K>
where
    S0: LegMatcher<Opposite = S1>,
    S1: LegMatcher,
    T0: UnsideQuantity<S0>,
    T1: UnsideQuantity<S1>,
{
    type Output = Tuple<T0::Output, T1::Output, K>;

    fn unsided(self) -> Self::Output {
        Tuple::new(self.0.unsided(), self.1.unsided())
    }
}
