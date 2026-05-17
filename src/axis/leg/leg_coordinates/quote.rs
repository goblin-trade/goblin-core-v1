use crate::{
    axis::leg::{leg_coordinates::LegCoordinates, Quote},
    quantities::{bits_layout::BitsLayout, Pos2, Position},
};

impl LegCoordinates for Quote {
    fn in_region(last_position: Pos2, position: Pos2) -> bool {
        position >= last_position
    }

    fn start<const BITS: u16>() -> Position {
        Position::ZERO
    }

    fn end<const BITS: u16>() -> Position {
        Position::new(BitsLayout::<BITS>::MAX)
    }
}
