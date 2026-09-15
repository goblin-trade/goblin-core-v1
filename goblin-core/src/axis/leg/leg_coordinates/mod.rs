mod base;
mod quote;

use crate::{axis::leg::LegQuantities, quantities::PositionV2};

pub trait LegCoordinates: LegQuantities {
    fn in_region(last_position: PositionV2, position: PositionV2) -> bool;

    fn start<const BITS: u16>() -> PositionV2;

    fn end<const BITS: u16>() -> PositionV2;
}
