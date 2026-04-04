use crate::{
    axis::leg::{leg_coordinates::LegCoordinates, Quote},
    matching::{bitmap::Coordinate, region::take_region::TakeRegion},
    quantities::Ticks,
};

impl LegCoordinates for Quote {
    fn closer_to_opposite_pole<K: PartialEq + PartialOrd>(first: K, second: K) -> bool {
        // For In=Quote (bid), we match upwards against resting asks
        first < second
    }

    fn region(limit_price: Ticks, price: Ticks) -> TakeRegion {
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
