use crate::axis::leg::leg_matcher::LegMatcher;

#[derive(Clone, Copy)]
pub struct QuotePair<In>
where
    In: LegMatcher,
{
    pub quote: In::MatchingLots,
    pub quote_opposite: <In::Opposite as LegMatcher>::MatchingLots,
}
