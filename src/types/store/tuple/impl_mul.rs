use core::ops::Mul;

use crate::types::Tuple;

impl<T0, T1, K, Rhs> Mul<Rhs> for Tuple<T0, T1, K>
where
    T0: Mul<Rhs>,
    T1: Mul<Rhs>,
    Rhs: Copy,
{
    type Output = Tuple<T0::Output, T1::Output, K>;

    fn mul(self, rhs: Rhs) -> Self::Output {
        Tuple::new(self.0 * rhs, self.1 * rhs)
    }
}
