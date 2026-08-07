use crate::{axis::leg::leg_matcher::LegMatcher, quantities::Position};

pub struct TakeHeaderOptional<In: LegMatcher> {
    /// The minimum number of base lots to fill, otherwise the order will be invalidated.
    pub min_lots_to_fill: In::Lots,

    /// The worst position to be matched against. Stop matching after this price is crossed.
    pub limit: Position,
}
