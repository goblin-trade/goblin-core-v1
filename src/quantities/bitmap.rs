use crate::define_custom_types;

// Inner bitmap
define_custom_types!(InnerBitmapIndex<u32>);
define_custom_types!(InnerIndex<u8>);
define_custom_types!(Row<u8>);
define_custom_types!(Column<u8>);

impl InnerIndex {
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

// Outer bitmap
define_custom_types!(OuterBitmapIndex<u32>);
define_custom_types!(OuterIndex<u8>);

impl InnerBitmapIndex {
    pub fn outer_bitmap_index(&self) -> OuterBitmapIndex {
        // divide by 256 → right shift 8 bits
        OuterBitmapIndex(self.0 >> 8)
    }

    pub fn outer_index(&self) -> OuterIndex {
        // modulo 256 → mask lowest 8 bits
        OuterIndex((self.0 & 0b1111_1111) as u8)
    }
}
