use crate::{
    axis::leg::{Base, LegCoordinates},
    quantities::{BitsLayout, FullPosition, PositionV2},
};

impl LegCoordinates for Base {
    fn in_region(last_position: PositionV2, position: PositionV2) -> bool {
        position <= last_position
    }

    fn start<const BITS: u16>() -> PositionV2 {
        PositionV2::new(BitsLayout::<BITS>::MAX)
    }

    fn end<const BITS: u16>() -> PositionV2 {
        PositionV2::ZERO
    }
}
