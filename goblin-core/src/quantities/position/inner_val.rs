use core::ops::{Add, Shr};

use deku::DekuReader;

pub trait InnerVal:
    Clone + Copy + Sized + Shr + Into<u64> + Add<Output = Self> + for<'a> DekuReader<'a>
{
    /// Convert from u64, truncating if needed
    ///
    /// # Safety
    ///
    /// Truncating u64 to u8 is intended behavior
    fn truncate_from_u64(val: u64) -> Self;

    /// Decode from the low bits of a raw bit field, truncating to the inner width.
    fn from_raw(raw: u64) -> Self;
}

impl InnerVal for u8 {
    fn truncate_from_u64(v: u64) -> Self {
        v as u8
    }

    fn from_raw(raw: u64) -> Self {
        raw as u8
    }
}

impl InnerVal for u64 {
    fn truncate_from_u64(v: u64) -> Self {
        v
    }

    fn from_raw(raw: u64) -> Self {
        raw
    }
}

impl InnerVal for u32 {
    fn truncate_from_u64(v: u64) -> Self {
        v as u32
    }

    fn from_raw(raw: u64) -> Self {
        raw as u32
    }
}
