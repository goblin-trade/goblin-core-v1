use crate::{
    axis::{leg::leg_matcher::LegMatcher, market::LotSizePair},
    settlement::{local_delta::MakerDeltaPair, MatchedAtoms, MatchedLotsPair},
};

/// Pending update to maker state for the token at In
/// Eg. for In: Base, these updates will apply on the Base token maker delta
#[derive(Clone, Copy)]
pub struct GlobalMakerUpdate<In: LegMatcher> {
    /// Matched atoms
    pub matched_atoms: MatchedAtoms<In>,
}

impl<In> GlobalMakerUpdate<In>
where
    In: LegMatcher,
{
    pub fn new(maker_delta_pair: &MakerDeltaPair, lot_size_pair: &LotSizePair) -> Self {
        let matched_lots_pair = MatchedLotsPair::from(maker_delta_pair);
        let matched_atoms = MatchedAtoms::new(&matched_lots_pair, lot_size_pair);

        Self { matched_atoms }
    }
}
