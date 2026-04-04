use crate::{
    axis::leg::leg_quantities::LegQuantities,
    matching::{bitmap::Coordinate, region::take_region::TakeRegion},
    quantities::Ticks,
};

pub trait LegCoordinates: LegQuantities {
    fn take_region(limit_price: Ticks, price: Ticks) -> TakeRegion;

    fn start_value<C: Coordinate>() -> C;

    fn end_value<C: Coordinate>() -> C;
}
