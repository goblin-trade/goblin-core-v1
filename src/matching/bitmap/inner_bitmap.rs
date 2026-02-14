use crate::{
    define_custom_type,
    matching::bitmap::{OuterBitmapIndex, OuterPos},
};

// Inner bitmap
define_custom_type!(InnerBitmapIndex<u64>);
define_custom_type!(InnerPos<u8>);
define_custom_type!(Row<u8>);
define_custom_type!(Column<u8>);

impl InnerPos {
    pub fn row(&self) -> Row {
        // divide by 8 → right shift 3 bits
        Row(self.0 >> 3)
    }

    pub fn column(&self) -> Column {
        // modulo 8 → mask lowest 3 bits
        Column(self.0 & 0b111)
    }

    pub fn from_row_column(row: Row, column: Column) -> Self {
        // row * 8 + column
        Self((row.0 << 3) | column.0)
    }
}

impl InnerBitmapIndex {
    pub fn outer_bitmap_index(&self) -> OuterBitmapIndex {
        // divide by 256 → right shift 8 bits
        OuterBitmapIndex(self.0 >> 8)
    }

    pub fn outer_pos(&self) -> OuterPos {
        // modulo 256 → mask lowest 8 bits
        OuterPos((self.0 & 0b1111_1111) as u8)
    }
}
