use crate::{axis::leg::leg_quantities::LegQuantities, matching::bitmap::Coordinate};

pub trait LegCoordinates: LegQuantities {
    /// Whether `first` is closer to the centre than `second`
    ///
    /// # Convention
    ///
    /// `In` is the taker direction
    ///
    /// 1. For In = Base (ask / sell) match against resting bids downwards. first > second.
    /// 2. For In = Quote (bid / buy) match against resting asks upwards. first < second.
    fn closer_to_centre<K: PartialEq + PartialOrd>(first: K, second: K) -> bool;

    fn start_value<C: Coordinate>() -> C;
}
