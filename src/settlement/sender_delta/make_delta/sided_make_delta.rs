use crate::{
    axis::leg::{leg_matcher::LegMatcher, leg_math::LegMath},
    settlement::MakeDelta,
};

pub type SidedMakeDeltaV2<In: LegMatcher> = MakeDelta<<In::Opposite as LegMath>::MatchingLots>;
