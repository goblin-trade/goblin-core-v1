use crate::quantities::DerivedPosition;

pub trait BitmapPosition {
    fn byte_index(&self) -> usize;
    fn bit_index(&self) -> usize;
}

impl<const BITS: u16> BitmapPosition for DerivedPosition<u8, BITS> {
    fn byte_index(&self) -> usize {
        self.inner as usize / 8
    }

    fn bit_index(&self) -> usize {
        self.inner as usize % 8
    }
}
