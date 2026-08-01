use crate::{
    quantities::QuantityOps,
    settlement::{CheckedOps, ConstZero},
};

impl QuantityOps for i64 {
    const MIN: Self = 0;
    const MAX: Self = i64::MAX;
    const ONE: Self = 1;
}

impl ConstZero for i64 {
    const ZEROED: Self = 0;
}

impl CheckedOps for i64 {
    fn checked_add(self, rhs: Self) -> Option<Self> {
        self.checked_add(rhs)
    }

    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.checked_sub(rhs)
    }
}
