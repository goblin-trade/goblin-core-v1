use core::ops::{Add, Shr};

use crate::input_processor::FixedDecode;

pub trait InnerVal: Sized + Shr + Into<u64> + Add<Output = Self> + for<'a> FixedDecode<'a> {
    /// Convert from u64, truncating if needed
    ///
    /// # Safety
    ///
    /// Truncating u64 to u8 is intended behavior
    fn truncate_from_u64(val: u64) -> Self;
}

impl InnerVal for u8 {
    fn truncate_from_u64(v: u64) -> Self {
        v as u8
    }
}

impl InnerVal for u64 {
    fn truncate_from_u64(v: u64) -> Self {
        v
    }
}
