use crate::{quantities::QuantityOps, settlement::ConstZero};

impl ConstZero for u64 {
    const ZEROED: Self = 0;
}

// impl QuantityOps for u64 {
//     const MIN: Self = 0;

//     const MAX: Self = u64::MAX;

//     const ONE: Self = 1;

//     fn checked_sub(self, rhs: Self) -> Option<Self> {
//         todo!()
//     }
// }
