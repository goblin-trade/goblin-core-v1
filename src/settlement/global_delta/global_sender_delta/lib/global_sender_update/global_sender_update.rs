use crate::{
    axis::{leg::leg_matcher::LegMatcher, market::LotSizePair},
    settlement::{MatchedAtoms, MatchedLotsPair},
};

/// A balance update in the global token level namespace
///
/// Unlike TakerDelta which holds updates for tokens on both side,
/// GlobalUpdate represents balance updates for one token.
#[derive(Default, PartialEq, Clone, Copy)]
pub struct GlobalSenderUpdate<In: LegMatcher> {
    /// Matched atoms for the given token
    pub matched_atoms: MatchedAtoms<In>,
}

impl<In> GlobalSenderUpdate<In>
where
    In: LegMatcher,
{
    pub fn new(taker_delta_pair: &MatchedLotsPair, lot_size_pair: &LotSizePair) -> Self {
        let matched_atoms = MatchedAtoms::new(taker_delta_pair, lot_size_pair);
        Self { matched_atoms }
    }
}
