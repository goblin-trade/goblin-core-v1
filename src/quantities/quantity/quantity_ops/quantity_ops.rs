use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::settlement::{CheckedOps, ConstZero};

/// Blanket trait for all supported Quantity operations
///
/// Mul, Div and Mod is only implemented on a subset of Exp, so we cannot have a blanket
/// bound here.
///
/// Hack- implement QuantityOps on i64 and u64 for clean API
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
    + ConstZero
    + CheckedOps
{
    const MIN: Self;
    const MAX: Self;
    const ONE: Self;
}
