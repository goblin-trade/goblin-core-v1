use crate::{quantities::Ticks, types::Side};

pub struct MatchIterator {
    pub side: Side,
    pub best_price: Ticks,
}

// impl Iterator for MatchIterator {

// }
