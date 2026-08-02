use crate::{
    axis::leg::{leg_quantities::LegQuantities, LegEnum},
    quantities::Position,
};

pub trait LegCoordinates: LegQuantities {
    const LEG_ENUM: LegEnum;

    fn in_region(last_position: Position, position: Position) -> bool;

    fn start<const BITS: u16>() -> Position;

    fn end<const BITS: u16>() -> Position;
}
