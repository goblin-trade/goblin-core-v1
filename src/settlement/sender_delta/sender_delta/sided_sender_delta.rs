use crate::{
    axis::leg::{leg_matcher::LegMatcher, leg_math::LegMath},
    settlement::SenderDelta,
};

pub type SidedSenderDeltaV2<In: LegMatcher> = SenderDelta<
    <In as LegMath>::MatchingLots,
    <<In as LegMath>::Opposite as LegMath>::MatchingLots,
>;
