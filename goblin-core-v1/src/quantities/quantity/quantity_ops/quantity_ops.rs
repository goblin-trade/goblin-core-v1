use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::{
    input_processor::{Decodable, DecodablePrimitive, FixedDecode},
    settlement::{CheckedOps, ConstZero},
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
    + ConstZero
    + CheckedOps
    + DecodablePrimitive
    + Decodable
    + for<'a> FixedDecode<'a>
{
    const MIN: Self;
    const MAX: Self;
    const ONE: Self;
}
