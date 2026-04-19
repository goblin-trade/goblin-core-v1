use crate::{
    axis::leg::leg_quantities::LegQuantities,
    matching::region::take_region::TakeRegion,
    quantities::{Position, TickPosV2},
};

pub trait LegCoordinates: LegQuantities {
    fn take_region(limit_price: TickPosV2, price: TickPosV2) -> TakeRegion;

    fn start<const BITS: u16>() -> Position;

    fn end<const BITS: u16>() -> Position;
}
