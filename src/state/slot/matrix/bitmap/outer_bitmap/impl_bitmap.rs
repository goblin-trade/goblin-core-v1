use crate::{
    matching::bitmap::outer_pos::OuterPos,
    state::bitmap::{outer_bitmap::active_outer_bitmap::ActiveOuterBitmap, Bitmap},
};

impl Bitmap<OuterPos> for ActiveOuterBitmap {
    fn active(&self, pos: OuterPos) -> bool {
        let idx = pos.inner as usize;

        let byte_index = idx / 8;
        let bit_index = idx % 8;

        let byte = self.inner[byte_index];
        (byte >> bit_index) & 1 == 1
    }
}
