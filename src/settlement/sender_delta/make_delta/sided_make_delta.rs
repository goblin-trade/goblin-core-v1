use crate::{
    axis::leg::leg_matcher::LegMatcher,
    goblin_error::GoblinError,
    quantities::{BaseLots, Position, QuoteLotsPerBaseUnitPerTick, Ticks},
    settlement::{CheckedAdd, MakeDelta},
};

pub type SidedMakeDeltaV2<In: LegMatcher> = MakeDelta<<In::Opposite as LegMatcher>::MatchingLots>;
