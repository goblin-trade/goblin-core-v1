use core::ops::Mul;

use crate::quantities::{Exp, Quantity, QuantityOps};
use crate::types::Tuple;

impl<E, I, T0, T1, K> Mul<Tuple<T0, T1, K>> for Quantity<E, I>
where
    E: Exp,
    I: QuantityOps,
    Quantity<E, I>: Mul<T0> + Mul<T1>,
{
    type Output =
        Tuple<<Quantity<E, I> as Mul<T0>>::Output, <Quantity<E, I> as Mul<T1>>::Output, K>;

    fn mul(self, rhs: Tuple<T0, T1, K>) -> Self::Output {
        Tuple::new(self * rhs.0, self * rhs.1)
    }
}
