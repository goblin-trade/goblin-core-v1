use core::ops::Neg;

use crate::quantities::{Exp, Quantity};

impl<E> Neg for &Quantity<E, i64>
where
    E: Exp,
{
    type Output = Quantity<E, i64>;

    fn neg(self) -> Self::Output {
        Self::Output::new(-self.inner)
    }
}
