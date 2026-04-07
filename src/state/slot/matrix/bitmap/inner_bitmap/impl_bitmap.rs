use crate::{
    quantities::{ColumnV2, InnerPosV2, OuterBitmapIndexV2, OuterPosV2, RowV2},
    state::bitmap::{bitmap_iterator::BitmapIterator, inner_bitmap::InnerBitmap, Bitmap},
};

impl Bitmap<(OuterBitmapIndexV2, OuterPosV2), InnerPosV2> for InnerBitmap {
    fn pos_active(&self, pos: InnerPosV2) -> bool {
        let row_byte = self.inner[RowV2::from(pos).inner as usize];
        let mask = 1 << ColumnV2::from(pos).inner;
        (row_byte & mask) != 0
    }

    fn deactivate(&mut self, pos: InnerPosV2) {
        // mask with 0 at target bit, 1 elsewhere
        let mask = !(1u8 << ColumnV2::from(pos).inner);
        self.inner[RowV2::from(pos).inner as usize] &= mask;
    }
}

impl BitmapIterator<(OuterBitmapIndexV2, OuterPosV2), InnerPosV2> for InnerBitmap {}
