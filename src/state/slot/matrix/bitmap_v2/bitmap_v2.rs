use crate::quantities::DerivedPosition;

#[derive(Default)]
pub struct BitmapV2<const BITS: u16> {
    pub inner: [u8; 32],
}

impl<const BITS: u16> BitmapV2<BITS> {
    pub fn is_closed(&self) -> bool {
        const CLOSED_SENTINEL: [u8; 32] = [0xFF; 32];
        self.inner == CLOSED_SENTINEL
    }

    pub fn is_empty(&self) -> bool {
        const EMPTY_VALUE: [u8; 32] = [0; 32];
        self.inner == EMPTY_VALUE
    }

    pub fn is_active(&self) -> bool {
        !self.is_empty() && !self.is_closed()
    }

    pub fn index_active(&self, index: DerivedPosition<u8, BITS>) -> bool {
        let byte_index = index.inner as usize / 8;
        let bit_index = index.inner as usize % 8;

        let byte = self.inner[byte_index];
        let mask = 1 << bit_index;

        (byte & mask) != 0
    }
}
