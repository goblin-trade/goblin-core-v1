use core::ops::Div;

use crate::{
    quantities::{Exp, Quantity, QuantityOps},
    types::Tuple,
};

impl<E, I, T0, T1, K> Div<Tuple<T0, T1, K>> for Quantity<E, I>
where
    E: Exp,
    I: QuantityOps,
    Quantity<E, I>: Div<T0> + Div<T1>,
{
    type Output =
        Tuple<<Quantity<E, I> as Div<T0>>::Output, <Quantity<E, I> as Div<T1>>::Output, K>;

    fn div(self, rhs: Tuple<T0, T1, K>) -> Self::Output {
        Tuple::new(self / rhs.0, self / rhs.1)
    }
}

// impl<T0, T1, K, Lhs> Div<Tuple<T0, T1, K>> for Lhs
// where
//     Lhs: Div<T0> + Div<T1> + Copy,
// {
//     type Output = Tuple<<Lhs as Div<T0>>::Output, <Lhs as Div<T1>>::Output, K>;

//     fn div(self, rhs: Tuple<T0, T1, K>) -> Self::Output {
//         Tuple::new(self / rhs.0, self / rhs.1)
//     }
// }

// impl<T0, T1, K, Rhs> Div<Rhs> for Tuple<T0, T1, K>
// where
//     T0: Div<Rhs>,
//     T1: Div<Rhs>,
//     Rhs: Copy,
// {
//     type Output = Tuple<T0::Output, T1::Output, K>;

//     fn div(self, rhs: Rhs) -> Self::Output {
//         Tuple::new(self.0 / rhs, self.1 / rhs)
//     }
// }
