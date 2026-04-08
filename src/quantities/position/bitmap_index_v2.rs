use crate::quantities::{ordered_index::OrderedIndex, InnerPosV2, OuterPosV2};

pub trait BitmapIndexV2: OrderedIndex {
    fn inner(&self) -> u8;
    fn byte_index(&self) -> usize {
        self.inner() as usize / 8
    }
    fn bit_index(&self) -> usize {
        self.inner() as usize % 8
    }
}

impl BitmapIndexV2 for OuterPosV2 {
    fn inner(&self) -> u8 {
        self.inner
    }
}

impl BitmapIndexV2 for InnerPosV2 {
    fn inner(&self) -> u8 {
        self.inner
    }
}
