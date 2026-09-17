use crate::{input_processor::BitPack, types::Tuple};

/// A `Tuple` packs as `T0` followed by `T1` in the next `T0::CAPACITY` bits.
///
/// This is what lets a `SamePair<bool>` (i.e. `Tuple<bool, bool, Leg>`) be used
/// as a sub-byte field in a `#[fixed_codec(bits = N)]` struct.
impl<T0, T1, K> BitPack for Tuple<T0, T1, K>
where
    T0: BitPack,
    T1: BitPack,
    K: Copy,
{
    const CAPACITY: u8 = T0::CAPACITY + T1::CAPACITY;

    #[inline]
    fn to_raw(self) -> u64 {
        T0::to_raw(self.0) | (T1::to_raw(self.1) << T0::CAPACITY)
    }

    #[inline]
    fn from_raw(raw: u64) -> Self {
        Self::new(T0::from_raw(raw), T1::from_raw(raw >> T0::CAPACITY))
    }
}
