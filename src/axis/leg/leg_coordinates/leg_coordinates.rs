use crate::{
    axis::leg::leg_quantities::LegQuantities,
    matching::region::take_region::TakeRegion,
    quantities::{Position, TickPos},
};

pub trait LegCoordinates: LegQuantities {
    fn take_region(limit_price: TickPos, price: TickPos) -> TakeRegion;

    fn start<const BITS: u16>() -> Position;

    fn end<const BITS: u16>() -> Position;
}
