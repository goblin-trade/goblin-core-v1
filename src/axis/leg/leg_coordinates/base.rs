use crate::{
    axis::leg::{leg_coordinates::LegCoordinates, Base},
    matching::bitmap::Coordinate,
};

impl LegCoordinates for Base {
    fn closer_to_centre<K: PartialEq + PartialOrd>(first: K, second: K) -> bool {
        // For In=Base (ask), we match downwards against resting bids
        first > second
    }

    fn start_value<C: Coordinate>() -> C {
        C::MAX
    }
}
