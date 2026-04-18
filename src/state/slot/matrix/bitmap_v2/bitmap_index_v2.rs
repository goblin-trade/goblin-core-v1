use crate::quantities::DerivedPosition;

pub type BitmapIndexV2<const BITS: u16> = DerivedPosition<u8, BITS>;

impl<const BITS: u16> BitmapIndexV2<BITS> {
    pub fn byte_index(&self) -> usize {
        self.inner as usize / 8
    }

    pub fn bit_index(&self) -> usize {
        self.inner as usize % 8
    }
}
