use crate::{
    axis::leg::{Base, LegCoordinates},
    quantities::{BitsLayout, FullPos, FullPosition},
};

impl LegCoordinates for Base {
    fn in_region(last_position: FullPos, position: FullPos) -> bool {
        position <= last_position
    }

    fn start<const BITS: u16>() -> FullPos {
        FullPos::new(BitsLayout::<BITS>::MAX)
    }

    fn end<const BITS: u16>() -> FullPos {
        FullPos::ZERO
    }
}
