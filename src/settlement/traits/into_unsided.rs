use crate::axis::{leg::leg_matcher::LegMatcher, market::LotSizePair};

pub trait IntoUnsided<In>
where
    In: LegMatcher,
{
    type Unsided;

    fn into_insided(&self, lot_size_pair: &LotSizePair) -> Self::Unsided;
}
