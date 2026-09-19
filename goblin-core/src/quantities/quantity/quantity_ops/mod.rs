mod impl_i32;
mod impl_i64;
mod impl_u32;
mod impl_u64;

use core::ops::{Add, AddAssign, Sub, SubAssign};

use deku::DekuReader;
#[cfg(feature = "encode")]
use deku::DekuWriter;

use crate::{
    input_processor::FixedCodec,
    settlement::{CheckedOps, ConstDefault},
};

/// Encoder side of a quantity, required only when the `encode` feature is on.
///
/// This exists so [`QuantityOps`] can require `DekuWriter` on encode builds
/// without a `where`-clause attribute (an unstable feature that rust-analyzer
/// cannot parse).
#[cfg(feature = "encode")]
pub trait MaybeDekuEncode: DekuWriter {}
#[cfg(feature = "encode")]
impl<T: DekuWriter> MaybeDekuEncode for T {}

/// Encoder side of a quantity; a no-op when `encode` is disabled.
#[cfg(not(feature = "encode"))]
pub trait MaybeDekuEncode {}
#[cfg(not(feature = "encode"))]
impl<T> MaybeDekuEncode for T {}

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
    + FixedCodec
    + for<'a> DekuReader<'a>
    + MaybeDekuEncode
{
    const MIN: Self;
    const MAX: Self;
    const ONE: Self;
}
