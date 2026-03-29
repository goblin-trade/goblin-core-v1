use crate::{
    matching::bitmap::{column::Column, inner_pos::InnerPos, row::Row},
    state::bitmap::{inner_bitmap::InnerBitmap, Bitmap},
};

impl Bitmap<InnerPos> for InnerBitmap {
    fn pos_active(&self, pos: InnerPos) -> bool {
        let row_byte = self.inner[Row::from(pos).inner as usize];
        let mask = 1 << Column::from(pos).inner;
        (row_byte & mask) != 0
    }

    fn deactivate(&mut self, pos: InnerPos) {
        // mask with 0 at target bit, 1 elsewhere
        let mask = !(1u8 << Column::from(pos).inner);
        self.inner[Row::from(pos).inner as usize] &= mask;
    }
}
