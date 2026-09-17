use crate::{
    input_processor::BitPack,
    quantities::{Exp, Quantity, QuantityOps},
};

/// `BitPack` mirrors the inner integer, so a `Quantity` used as a wire bit field
/// (e.g. a 30-bit `BaseLots<u32>`) packs exactly like its underlying value.
impl<E, I> BitPack for Quantity<E, I>
where
    E: Exp + Copy,
    I: QuantityOps + BitPack,
{
    const CAPACITY: u8 = I::CAPACITY;

    #[inline]
    fn to_raw(self) -> u64 {
        self.inner.to_raw()
    }

    #[inline]
    fn from_raw(raw: u64) -> Self {
        Self::new(I::from_raw(raw))
    }
}
