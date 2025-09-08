use crate::{
    define_custom_type,
    quantities::{InnerBitmapIndex, Row},
};

define_custom_type!(Ticks<u32>);

impl Ticks {
    pub fn inner_bitmap_index(&self) -> InnerBitmapIndex {
        // divide by 32 → right shift 5 bits
        InnerBitmapIndex(self.0 >> 5)
    }

    pub fn row(&self) -> Row {
        // modulo 32 → mask lowest 5 bits
        Row((self.0 & 0b11111) as u8)
    }

    pub fn from_inner_bitmap_index_row(inner_bitmap_index: InnerBitmapIndex, row: Row) -> Self {
        // inner_bitmap_index * 32 + row
        Self((inner_bitmap_index.0 << 5) | row.0 as u32)
    }
}
