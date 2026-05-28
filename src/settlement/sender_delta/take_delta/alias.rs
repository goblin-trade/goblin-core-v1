use crate::{
    axis::leg::leg_matcher::LegMatcher, quantities::UnsidedAtoms,
    settlement::sender_delta::TakeDelta,
};

pub type SidedTakeDeltaV2<In: LegMatcher> = TakeDelta<
    <In as LegMatcher>::MatchingLots,
    <<In as LegMatcher>::Opposite as LegMatcher>::MatchingLots,
>;

pub type UnsidedTakeDeltaV2 = TakeDelta<UnsidedAtoms, UnsidedAtoms>;
