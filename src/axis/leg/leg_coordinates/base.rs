use crate::{
    axis::leg::{leg_coordinates::LegCoordinates, Base, LegEnum},
    quantities::{bits_layout::BitsLayout, Position},
};

impl LegCoordinates for Base {
    const LEG_ENUM: LegEnum = LegEnum::Base;

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
