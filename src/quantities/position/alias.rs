use crate::quantities::DerivedPosition;

const fn bits(offset: u8, count: u8) -> u16 {
    ((offset as u16) << 8) | (count as u16)
}

pub const INNER_POS: u16 = bits(0, 8);
pub const COLUMN: u16 = bits(0, 3);
pub const ROW: u16 = bits(3, 5);
pub const OUTER_POS: u16 = bits(8, 8);
pub const OUTER_BITMAP_INDEX: u16 = bits(16, 48);
pub const TICK_POS: u16 = bits(3, 61);

pub const POS_0: u16 = OUTER_BITMAP_INDEX;
pub const POS_1: u16 = bits(8, 56);
pub const POS_2: u16 = bits(0, 64);

pub type InnerPos = DerivedPosition<u8, INNER_POS>;
pub type Column = DerivedPosition<u8, COLUMN>;
pub type Row = DerivedPosition<u8, ROW>;
pub type OuterPos = DerivedPosition<u8, OUTER_POS>;
pub type OuterBitmapIndex = DerivedPosition<u64, OUTER_BITMAP_INDEX>;
pub type TickPos = DerivedPosition<u64, TICK_POS>;
