use crate::quantities::DerivedPosition;

pub type InnerPosV2 = DerivedPosition<u8, 0, 8>;
pub type ColumnV2 = DerivedPosition<u8, 0, 3>;
pub type RowV2 = DerivedPosition<u8, 3, 5>;

pub type OuterPosV2 = DerivedPosition<u8, 8, 8>;
pub type OuterBitmapIndexV2 = DerivedPosition<u64, 16, 48>;
pub type TickPosV2 = DerivedPosition<u64, 3, 61>;
