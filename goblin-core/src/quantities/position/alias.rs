use crate::quantities::Position;

const fn bits(offset: u8, count: u8) -> u16 {
    ((offset as u16) << 8) | (count as u16)
}

pub const COLUMN: u16 = bits(0, 3);
pub const INNER_POS: u16 = bits(0, 8);
pub const ROW: u16 = bits(3, 5);
pub const OUTER_POS: u16 = bits(8, 8);
pub const OUTER_BITMAP_INDEX: u16 = bits(16, 48);
pub const TICK_POS: u16 = bits(3, 61);

pub const POS_0: u16 = OUTER_BITMAP_INDEX;
pub const POS_1: u16 = bits(8, 56);
pub const POS_2: u16 = bits(0, 64);

pub type InnerPos = Position<u8, INNER_POS>;
pub type Column = Position<u8, COLUMN>;
pub type Row = Position<u8, ROW>;
pub type OuterPos = Position<u8, OUTER_POS>;

pub type TickPos = Position<u64, TICK_POS>;

pub type OuterBitmapIndex = Position<u64, OUTER_BITMAP_INDEX>;
pub type OuterBitmapIndexU32 = Position<u32, OUTER_BITMAP_INDEX>;

pub type FullPos = Position<u64, POS_2>;
pub type FullPosU32 = Position<u32, POS_2>;
