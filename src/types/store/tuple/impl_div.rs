use core::ops::Div;

use crate::types::Tuple;

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
