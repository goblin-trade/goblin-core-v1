use crate::{
    axis::leg::{leg_coordinates::LegCoordinates, Quote},
    matching::bitmap::Coordinate,
};

impl LegCoordinates for Quote {
    fn closer_to_centre<K: PartialEq + PartialOrd>(first: K, second: K) -> bool {
        // For In=Quote (bid), we match upwards against resting asks
        first < second
    }

    fn start_value<C: Coordinate>() -> C {
        C::MIN
    }
}
