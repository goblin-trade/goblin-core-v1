use crate::{
    input_processor::{BitPack, bit_mask},
    quantities::{InnerVal, Position},
};

/// A `Position` packs as its inner integer, truncated to the bit count encoded
/// in `BITS` (`(offset << 8) | count`).
///
/// Using the declared count (rather than the full inner width) means a
/// `Position<u8, bits(0, 8)>` like `InnerPos` reports 8 bits, while a narrower
/// one like `Column` (3 bits) reports 3 — so a leading "full space" field takes
/// exactly its wire width.
impl<K, const BITS: u16> BitPack for Position<K, BITS>
where
    K: InnerVal + BitPack,
{
    const CAPACITY: u8 = (BITS & 0xFF) as u8;

    #[inline]
    fn to_raw(self) -> u64 {
        self.inner.to_raw() & bit_mask(Self::CAPACITY)
    }

    #[inline]
    fn from_raw(raw: u64) -> Self {
        Self::new(K::from_raw(raw))
    }
}
