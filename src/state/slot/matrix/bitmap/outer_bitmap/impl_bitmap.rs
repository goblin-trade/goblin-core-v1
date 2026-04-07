use crate::{
    quantities::{OuterBitmapIndexV2, OuterPosV2},
    state::bitmap::{
        bitmap_iterator::BitmapIterator, outer_bitmap::active_outer_bitmap::ActiveOuterBitmap,
        Bitmap,
    },
};

impl Bitmap<OuterBitmapIndexV2, OuterPosV2> for ActiveOuterBitmap {
    fn pos_active(&self, pos: OuterPosV2) -> bool {
        let byte = self.inner[pos.byte_index()];
        let mask = 1 << pos.bit_index();

        (byte & mask) != 0
    }

    fn deactivate(&mut self, pos: OuterPosV2) {
        // mask with 0 at target bit, 1 elsewhere
        let mask = !(1u8 << pos.bit_index());
        self.inner[pos.byte_index()] &= mask;
    }
}

impl BitmapIterator<OuterBitmapIndexV2, OuterPosV2> for ActiveOuterBitmap {}
