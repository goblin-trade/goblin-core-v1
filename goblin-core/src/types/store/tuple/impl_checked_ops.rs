use crate::{settlement::CheckedOps, types::Tuple};

impl<T0, T1, K> CheckedOps for Tuple<T0, T1, K>
where
    T0: CheckedOps,
    T1: CheckedOps,
{
    fn checked_add(self, rhs: Self) -> Option<Self> {
        Some(Tuple::new(
            self.0.checked_add(rhs.0)?,
            self.1.checked_add(rhs.1)?,
        ))
    }

    fn checked_sub(self, rhs: Self) -> Option<Self> {
        Some(Tuple::new(
            self.0.checked_sub(rhs.0)?,
            self.1.checked_sub(rhs.1)?,
        ))
    }

    fn checked_mul(self, rhs: Self) -> Option<Self> {
        Some(Tuple::new(
            self.0.checked_mul(rhs.0)?,
            self.1.checked_mul(rhs.1)?,
        ))
    }
}
