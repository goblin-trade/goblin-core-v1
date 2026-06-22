use crate::{
    quantities::QuantityOps,
    settlement::{CheckedOps, ConstZero},
};

impl ConstZero for u64 {
    const ZEROED: Self = 0;
}

impl CheckedOps for u64 {
    fn checked_add(self, rhs: Self) -> Option<Self> {
        self.checked_add(rhs)
    }

    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.checked_sub(rhs)
    }
}

impl QuantityOps for u64 {
    const MIN: Self = 0;
    const MAX: Self = u64::MAX;
    const ONE: Self = 1;
}
