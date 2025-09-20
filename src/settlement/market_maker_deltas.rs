use crate::{
    goblin_error::GoblinError,
    types::{Address, Base, LegMarker, Quote},
    utils::FixedMap,
};

pub const MAX_MAKERS: usize = 16;
pub type MarketMakerDeltas = FixedMap<Address, MakerDelta, MAX_MAKERS>;

#[derive(Default)]
pub struct MakerDelta {
    pub bid: MakerSideDelta<Quote>,
    pub ask: MakerSideDelta<Base>,
}

impl MakerDelta {
    /// Accumulate the matched lots for a given maker on a given side.
    ///
    /// This does not immediately update balances. Instead, it records the
    /// pending effect of a match so that all changes can be applied together
    /// in the settlement phase.
    ///
    /// # Arguments
    /// - `lots`: Lots gained by the maker (credited).
    /// - `lots_opposite`: Lots lost by the maker on the opposite side (debited).
    ///
    /// # Overflow
    /// - `free_lots_in` (credited lots) may overflow; this only affects
    ///   the maker’s credited balance and does not harm solvency.
    /// - `locked_lots_out` (debited lots) must not overflow, but is safe
    ///   since resting orders are always backed by reserves.
    pub fn accumulate_match_result<In: LegMarker>(
        &mut self,
        free_matching_lots_in: In::MatchingLots,
        locked_matching_lots_out: <In::Opposite as LegMarker>::MatchingLots,
    ) -> Result<(), GoblinError> {
        let deltas_for_side = In::maker_side_delta_mut(self);

        deltas_for_side.free_matching_lots_in += free_matching_lots_in;
        deltas_for_side.locked_matching_lots_out += locked_matching_lots_out;

        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct MakerSideDelta<In: LegMarker> {
    pub free_matching_lots_in: In::MatchingLots,
    pub locked_matching_lots_out: <In::Opposite as LegMarker>::MatchingLots,
}

impl<In: LegMarker> Default for MakerSideDelta<In> {
    fn default() -> Self {
        Self {
            free_matching_lots_in: In::MatchingLots::default(),
            locked_matching_lots_out: <In::Opposite as LegMarker>::MatchingLots::default(),
        }
    }
}
