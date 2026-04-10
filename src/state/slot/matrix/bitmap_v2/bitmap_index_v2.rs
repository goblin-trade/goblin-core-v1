use crate::{quantities::DerivedPosition, state::bitmap_v2::ordered_index::OrderedIndex};

pub type BitmapIndexV2<const BIT_OFFSET: usize, const BIT_COUNT: usize> =
    DerivedPosition<u8, BIT_OFFSET, BIT_COUNT>;

impl<const BIT_OFFSET: usize, const BIT_COUNT: usize> BitmapIndexV2<BIT_OFFSET, BIT_COUNT>
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
// pub trait BitmapIndexV2: OrderedIndex {
//     fn inner(&self) -> u8;
//     fn byte_index(&self) -> usize {
//         self.inner() as usize / 8
//     }
//     fn bit_index(&self) -> usize {
//         self.inner() as usize % 8
//     }
// }

// impl BitmapIndexV2 for OuterPosV2 {
//     fn inner(&self) -> u8 {
//         self.inner
//     }
// }

// impl BitmapIndexV2 for InnerPosV2 {
//     fn inner(&self) -> u8 {
//         self.inner
//     }
// }
