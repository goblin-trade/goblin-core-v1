use crate::{
    axis::leg::leg_quantities::LegQuantities,
    quantities::{Pos2, Position},
};

pub trait LegCoordinates: LegQuantities {
    fn in_region(last_position: Pos2, position: Pos2) -> bool;

    fn start<const BITS: u16>() -> Position;

    fn end<const BITS: u16>() -> Position;
}
