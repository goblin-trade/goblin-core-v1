use crate::{
    axis::leg::{LegCoordinates, Quote},
    quantities::{BitsLayout, FullPos, FullPosition},
};

impl LegCoordinates for Quote {
    fn in_region(last_position: FullPos, position: FullPos) -> bool {
        position >= last_position
    }

    fn start<const BITS: u16>() -> FullPos {
        FullPos::ZERO
    }

    fn end<const BITS: u16>() -> FullPos {
        FullPos::new(BitsLayout::<BITS>::MAX)
    }
}
