use crate::{
    axis::leg::leg_matcher::LegMatcher,
    quantities::UnsidedAtoms,
    settlement::sender_delta::{MakeDelta, SenderDelta, TakeDelta},
};

pub type SidedTakeDeltaV2<In: LegMatcher> =
    TakeDelta<In::MatchingLots, <In::Opposite as LegMatcher>::MatchingLots>;

pub type UnsidedTakeDeltaV2 = TakeDelta<UnsidedAtoms, UnsidedAtoms>;

pub type SidedMakeDeltaV2<In: LegMatcher> = MakeDelta<<In::Opposite as LegMatcher>::MatchingLots>;
pub type UnsidedMakeDeltaV2 = MakeDelta<UnsidedAtoms>;

pub type SidedSenderDeltaV2<In: LegMatcher> = SenderDelta<
    <In as LegMatcher>::MatchingLots,
    <<In as LegMatcher>::Opposite as LegMatcher>::MatchingLots,
>;
pub type UnsidedSenderDeltaV2 = SenderDelta<UnsidedAtoms, UnsidedAtoms>;
