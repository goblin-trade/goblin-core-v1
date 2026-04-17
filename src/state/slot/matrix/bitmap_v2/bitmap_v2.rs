use crate::state::bitmap_v2::bitmap_index_v2::BitmapIndexV2;
use crate::state::bitmap_v2::ordered_index::OrderedIndex;

pub struct BitmapV2<const BITS: u16> {
    pub inner: [u8; 32],
}

impl<const BITS: u16> BitmapV2<BITS>
where
    BitmapIndexV2<BITS>: OrderedIndex,
{
    pub fn is_active(&self) -> bool {
        const EMPTY_VALUE: [u8; 32] = [0; 32];
        const CLOSED_SENTINEL: [u8; 32] = [0xFF; 32];

        self.inner != EMPTY_VALUE && self.inner != CLOSED_SENTINEL
    }

    pub fn index_active(&self, index: BitmapIndexV2<BITS>) -> bool {
        let byte = self.inner[index.byte_index()];
        let mask = 1 << index.bit_index();

        (byte & mask) != 0
    }
}
