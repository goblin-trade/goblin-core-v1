use crate::{
    axis::leg::{leg_coordinates::LegCoordinates, Base},
    quantities::{bits_layout::BitsLayout, Pos2, Position},
};

impl LegCoordinates for Base {
    fn in_region(last_position: Pos2, position: Pos2) -> bool {
        position <= last_position
    }

    fn start<const BITS: u16>() -> Position {
        Position::new(BitsLayout::<BITS>::MAX)
    }

    fn end<const BITS: u16>() -> Position {
        Position::ZERO
    }
}
