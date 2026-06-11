use crate::axis::{leg::leg_matcher::LegMatcher, market::LotSizePair};

pub trait UnsideDelta<In>
where
    In: LegMatcher,
{
    type Unsided;

    fn unside(&self, lot_size_pair: &LotSizePair) -> Self::Unsided;
}
