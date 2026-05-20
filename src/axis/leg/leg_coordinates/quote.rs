use crate::{
    axis::leg::{leg_coordinates::LegCoordinates, Quote},
    quantities::{bits_layout::BitsLayout, Position},
};

impl LegCoordinates for Quote {
    fn in_region(last_position: Position, position: Position) -> bool {
        position >= last_position
    }

    fn start<const BITS: u16>() -> Position {
        Position::ZERO
    }

    fn end<const BITS: u16>() -> Position {
        Position::new(BitsLayout::<BITS>::MAX)
    }
}
