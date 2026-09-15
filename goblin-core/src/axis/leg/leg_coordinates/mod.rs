mod base;
mod quote;

use crate::{axis::leg::LegQuantities, quantities::FullPos};

pub trait LegCoordinates: LegQuantities {
    fn in_region(last_position: FullPos, position: FullPos) -> bool;

    fn start<const BITS: u16>() -> FullPos;

    fn end<const BITS: u16>() -> FullPos;
}
