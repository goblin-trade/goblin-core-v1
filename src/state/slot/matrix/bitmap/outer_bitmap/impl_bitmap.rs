use crate::{
    matching::bitmap::{outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos},
    state::bitmap::{outer_bitmap::active_outer_bitmap::ActiveOuterBitmap, Bitmap},
};

impl Bitmap<OuterBitmapIndex, OuterPos> for ActiveOuterBitmap {
    fn pos_active(&self, pos: OuterPos) -> bool {
        let byte = self.inner[pos.byte_index()];
        let mask = 1 << pos.bit_index();

        (byte & mask) != 0
    }

    fn deactivate(&mut self, pos: OuterPos) {
        // mask with 0 at target bit, 1 elsewhere
        let mask = !(1u8 << pos.bit_index());
        self.inner[pos.byte_index()] &= mask;
    }
}
