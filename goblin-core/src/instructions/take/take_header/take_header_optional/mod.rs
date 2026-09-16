mod impl_variable_decode;

use crate::{axis::leg::LegMatcher, quantities::FullPos};

pub struct TakeHeaderOptional<In: LegMatcher> {
    /// The minimum number of base lots to fill, otherwise the order will be invalidated.
    /// TODO 32 bit
    pub min_lots_to_fill: In::Lots,

    /// The worst position to be matched against. Stop matching after this price is crossed.
    pub limit: FullPos,
}
