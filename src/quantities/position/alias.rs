use crate::quantities::DerivedPosition;

const fn bits(offset: u8, count: u8) -> u16 {
    ((offset as u16) << 8) | (count as u16)
}

pub const INNER_POS_V2: u16 = bits(0, 8);
pub const COLUMN_V2: u16 = bits(0, 3);
pub const ROW_V2: u16 = bits(3, 5);
pub const OUTER_POS_V2: u16 = bits(8, 8);
pub const OUTER_BITMAP_INDEX_V2: u16 = bits(16, 48);
pub const TICK_POS_V2: u16 = bits(3, 61);

pub type InnerPosV2 = DerivedPosition<u8, INNER_POS_V2>;
pub type ColumnV2 = DerivedPosition<u8, COLUMN_V2>;
pub type RowV2 = DerivedPosition<u8, ROW_V2>;
pub type OuterPosV2 = DerivedPosition<u8, OUTER_POS_V2>;
pub type OuterBitmapIndexV2 = DerivedPosition<u64, OUTER_BITMAP_INDEX_V2>;
pub type TickPosV2 = DerivedPosition<u64, TICK_POS_V2>;
