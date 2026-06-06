use crate::{
    axis::leg::leg_math::LegMath, quantities::UnsidedAtoms, settlement::sender_delta::TakeDelta,
};

pub type SidedTakeDeltaV2<In: LegMath> =
    TakeDelta<<In as LegMath>::MatchingLots, <<In as LegMath>::Opposite as LegMath>::MatchingLots>;

pub type UnsidedTakeDeltaV2 = TakeDelta<UnsidedAtoms, UnsidedAtoms>;
