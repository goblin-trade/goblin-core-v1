use crate::{
    axis::leg::leg_matcher::LegMatcher,
    quantities::UnsidedAtoms,
    settlement::sender_delta::{MakeDelta, SenderDelta, TakeDelta},
};

pub type LocalTakeDeltaV2<In: LegMatcher> =
    TakeDelta<In::MatchingLots, <In::Opposite as LegMatcher>::MatchingLots>;

pub type GlobalTakeDeltaV2 = TakeDelta<UnsidedAtoms, UnsidedAtoms>;

pub type LocalMakeDeltaV2<In: LegMatcher> = MakeDelta<<In::Opposite as LegMatcher>::MatchingLots>;
pub type GlobalMakeDeltaV2 = MakeDelta<UnsidedAtoms>;

pub type LocalSenderDeltaV2<In: LegMatcher> =
    SenderDelta<In::MatchingLots, <In::Opposite as LegMatcher>::MatchingLots>;
pub type GlobalSenderDeltaV2 = SenderDelta<UnsidedAtoms, UnsidedAtoms>;
