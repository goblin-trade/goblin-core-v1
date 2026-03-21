use crate::{
    matching::bitmap::{column::Column, inner_pos::InnerPos, row::Row},
    state::bitmap::{inner_bitmap::InnerBitmap, Bitmap},
};

impl Bitmap<InnerPos> for InnerBitmap {
    fn active(&self, pos: InnerPos) -> bool {
        let row = Row::from(pos);
        let column = Column::from(pos);

        let row_bits = self.inner[row.inner as usize];
        let mask = 1u8 << column.inner;
        (row_bits & mask) != 0
    }
}
