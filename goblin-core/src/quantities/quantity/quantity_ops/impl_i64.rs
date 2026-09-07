use crate::{quantities::QuantityOps, settlement::CheckedOps};

impl QuantityOps for i64 {
    const MIN: Self = 0;
    const MAX: Self = i64::MAX;
    const ONE: Self = 1;
}

impl CheckedOps for i64 {
    fn checked_add(self, rhs: Self) -> Option<Self> {
        self.checked_add(rhs)
    }

    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.checked_sub(rhs)
    }

    fn checked_mul(self, rhs: Self) -> Option<Self> {
        self.checked_mul(rhs)
    }
}
