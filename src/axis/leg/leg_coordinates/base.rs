use crate::{
    axis::leg::{leg_coordinates::LegCoordinates, Base},
    matching::{bitmap::Coordinate, region::take_region::TakeRegion},
    quantities::Ticks,
};

impl LegCoordinates for Base {
    fn closer_to_opposite_pole<K: PartialEq + PartialOrd>(first: K, second: K) -> bool {
        // For In=Base (ask), we match downwards against resting bids
        first > second
    }

    fn region(limit_price: Ticks, price: Ticks) -> TakeRegion {
        if price > limit_price {
            TakeRegion::NotLeg
        } else {
            TakeRegion::Leg
        }
    }

    fn start_value<C: Coordinate>() -> C {
        C::MAX
    }

    fn end_value<C: Coordinate>() -> C {
        C::MIN
    }
}
