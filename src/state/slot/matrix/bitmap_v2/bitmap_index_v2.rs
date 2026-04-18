use crate::{quantities::DerivedPosition, state::bitmap_v2::ordered_index::OrderedIndex};

pub type BitmapIndexV2<const BITS: u16> = DerivedPosition<u8, BITS>;

impl<const BITS: u16> BitmapIndexV2<BITS>
where
    Self: OrderedIndex,
{
    pub fn byte_index(&self) -> usize {
        self.inner as usize / 8
    }

    pub fn bit_index(&self) -> usize {
        self.inner as usize % 8
    }
}
