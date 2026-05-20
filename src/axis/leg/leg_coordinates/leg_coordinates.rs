use crate::{axis::leg::leg_quantities::LegQuantities, quantities::Position};

pub trait LegCoordinates: LegQuantities {
    fn in_region(last_position: Position, position: Position) -> bool;

    fn start<const BITS: u16>() -> Position;

    fn end<const BITS: u16>() -> Position;
}
