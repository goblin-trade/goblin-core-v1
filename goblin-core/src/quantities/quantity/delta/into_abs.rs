use crate::quantities::{Exp, Quantity};

pub trait IntoAbs<E>
where
    E: Exp,
{
    fn abs(&self) -> Quantity<E, u64>;
}

impl<E> IntoAbs<E> for Quantity<E, i64>
where
    E: Exp,
{
    fn abs(&self) -> Quantity<E, u64> {
        let inner = self.inner.unsigned_abs();
        Quantity::new(inner)
    }
}
