use crate::{axis::leg::leg_matcher::LegMatcher, settlement::SenderDelta};

pub type SidedSenderDeltaV2<In: LegMatcher> = SenderDelta<
    <In as LegMatcher>::MatchingLots,
    <<In as LegMatcher>::Opposite as LegMatcher>::MatchingLots,
>;
