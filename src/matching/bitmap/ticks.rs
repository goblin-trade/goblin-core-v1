use crate::{
    matching::bitmap::{InnerBitmapIndex, InnerPos, OuterBitmapIndex, OuterPos, Row},
    quantities::Ticks,
};

impl Ticks {
    pub fn inner_bitmap_index(&self) -> InnerBitmapIndex {
        // divide by 32 → right shift 5 bits
        InnerBitmapIndex(self.inner >> 5)
    }

    pub fn row(&self) -> Row {
        // modulo 32 → mask lowest 5 bits
        Row((self.inner & 0b11111) as u8)
    }

    pub fn from_matrix(
        outer_bitmap_index: OuterBitmapIndex,
        outer_pos: OuterPos,
        inner_pos: InnerPos,
    ) -> Self {
        let ticks_raw =
            outer_bitmap_index.0 * 256 + outer_pos.0 as u64 * 32 + inner_pos.0 as u64 / 8;
        Self::new(ticks_raw)
    }

    pub fn from_inner_bitmap(inner_bitmap_index: InnerBitmapIndex, row: Row) -> Self {
        // inner_bitmap_index * 32 + row
        Self::new((inner_bitmap_index.0 << 5) | row.0 as u64)
    }
}
