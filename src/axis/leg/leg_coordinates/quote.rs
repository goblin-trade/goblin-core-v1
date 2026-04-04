use crate::{
    axis::leg::{leg_coordinates::LegCoordinates, Quote},
    matching::{bitmap::Coordinate, region::take_region::TakeRegion},
    quantities::Ticks,
};

impl LegCoordinates for Quote {
    fn take_region(limit_price: Ticks, price: Ticks) -> TakeRegion {
        if price < limit_price {
            TakeRegion::NotLeg
        } else {
            TakeRegion::Leg
        }
    }

    fn start_value<C: Coordinate>() -> C {
        C::MIN
    }

    fn end_value<C: Coordinate>() -> C {
        C::MAX
    }
}
