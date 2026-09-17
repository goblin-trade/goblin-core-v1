//! Bit-level packing primitives.
//!
//! This module is intentionally independent of any byte buffer. It only knows
//! how to insert a scalar value into, and extract it from, a fixed-width bit
//! slice of an integer "lane" (a `u8`/`u16`/`u32`/`u64`). Reading those lanes
//! from, and writing them to, the wire is the responsibility of
//! [`fixed_codec`](super::fixed_codec).
//!
//! Splitting the two keeps the buffer codec free of shift/mask logic and keeps
//! the bit logic testable in isolation.

use core::marker::PhantomData;

use crate::goblin_error::GoblinError;

/// Mask with the lowest `bits` bits set.
///
/// `bits == 64` is special-cased because `1u64 << 64` is undefined.
#[inline]
pub const fn bit_mask(bits: u8) -> u64 {
    if bits >= 64 {
        u64::MAX
    } else {
        (1u64 << bits) - 1
    }
}

/// A value that can be packed into / unpacked from a sub-byte slice of an
/// integer lane.
///
/// `CAPACITY` is the *maximum* width of the type, not the width used by any
/// particular struct field. A field's actual width is declared with
/// `#[codec(bits = N)]` and must satisfy `N <= Self::CAPACITY`.
///
/// Implementors must be `Copy` — bit fields are scalars, and the derive macro
/// takes them by reference when writing a lane.
pub trait BitPack: Copy {
    /// Maximum number of bits this type can occupy on the wire.
    const CAPACITY: u8;

    /// Widen `self` into a raw integer. The low bits are the payload.
    fn to_raw(self) -> u64;

    /// Narrow the low bits of `raw` back into `Self`.
    fn from_raw(raw: u64) -> Self;

    /// Insert `self` into `lane` starting at bit `shift`, occupying `bits` bits.
    ///
    /// `bits` must not exceed [`Self::CAPACITY`].
    #[inline]
    fn pack_into(&self, lane: &mut u64, shift: u8, bits: u8) {
        debug_assert!(bits <= Self::CAPACITY, "field wider than type capacity");
        *lane |= (self.to_raw() & bit_mask(bits)) << shift;
    }

    /// Extract a value occupying `bits` bits starting at bit `shift` of `lane`.
    ///
    /// `bits` must not exceed [`Self::CAPACITY`].
    #[inline]
    fn unpack_from(lane: u64, shift: u8, bits: u8) -> Self {
        debug_assert!(bits <= Self::CAPACITY, "field wider than type capacity");
        Self::from_raw((lane >> shift) & bit_mask(bits))
    }

    /// Check constraints on an already-decoded value.
    ///
    /// This mirrors [`FixedCodec::validate`](super::FixedCodec::validate) for
    /// bit fields. It lives here rather than on `FixedCodec` so that a type can
    /// be used as a sub-byte field without also committing to a byte-aligned
    /// `ENCODED_SIZE` (which is target-dependent for `usize`).
    fn validate(&self) -> Result<(), GoblinError> {
        Ok(())
    }
}

macro_rules! impl_bit_pack_int {
    ($($t:ty),* $(,)?) => {$(
        impl BitPack for $t {
            const CAPACITY: u8 = (core::mem::size_of::<Self>() * 8) as u8;

            #[inline]
            fn to_raw(self) -> u64 {
                self as u64
            }

            #[inline]
            fn from_raw(raw: u64) -> Self {
                raw as Self
            }
        }
    )*};
}

impl_bit_pack_int!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

impl BitPack for bool {
    const CAPACITY: u8 = 1;

    #[inline]
    fn to_raw(self) -> u64 {
        self as u64
    }

    #[inline]
    fn from_raw(raw: u64) -> Self {
        (raw & 1) != 0
    }
}

/// `PhantomData` occupies no bits. This lets generic structs that carry a
/// `PhantomData<E>` field be derived without special-casing the marker.
impl<T> BitPack for PhantomData<T> {
    const CAPACITY: u8 = 0;

    #[inline]
    fn to_raw(self) -> u64 {
        0
    }

    #[inline]
    fn from_raw(_raw: u64) -> Self {
        PhantomData
    }
}
