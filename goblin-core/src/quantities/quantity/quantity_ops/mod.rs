mod impl_i64;
mod impl_u64;

use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::{
    input_processor::FixedDecode,
    settlement::{CheckedOps, ConstDefault},
};

/// Blanket trait for all supported Quantity operations
///
/// Mul, Div and Mod is only implemented on a subset of Exp, so we cannot have a blanket
/// bound here.
pub trait QuantityOps:
    Copy
    + Sized
    + PartialEq
    + Default
    + Add<Output = Self>
    + Sub<Output = Self>
    + AddAssign
    + SubAssign
    + PartialOrd
    + Ord
    + ConstDefault
    + CheckedOps
    + for<'a> FixedDecode<'a>
{
    const MIN: Self;
    const MAX: Self;
    const ONE: Self;
}
