use crate::{
    axis::leg::{Base, LegCoordinates},
    quantities::{BitsLayout, Position},
};

impl LegCoordinates for Base {
    fn in_region(last_position: Position, position: Position) -> bool {
        position <= last_position
    }

    fn start<const BITS: u16>() -> Position {
        Position::new(BitsLayout::<BITS>::MAX)
    }

    fn end<const BITS: u16>() -> Position {
        Position::ZERO
    }
}
