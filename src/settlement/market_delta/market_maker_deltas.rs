use crate::{
    goblin_error::GoblinError,
    settlement::market_delta::MakerDelta,
    types::{Address, Base, LegMarker, Pair, PairAccessor, Quote},
    utils::FixedMap,
};

pub const MAX_MAKERS: usize = 16;

/// Deltas of makers in the market namespace
///
/// This list tracks deltas generated when resting orders are matched.
pub type MarketMakerDeltas = FixedMap<Address, MakerDeltaPair, MAX_MAKERS>;

/// Maker deltas for base and quote sides
pub type MakerDeltaPair = Pair<MakerDelta<Base>, MakerDelta<Quote>>;

impl MakerDeltaPair {
    /// Accumulate the matched lots for a given maker on a given side.
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
    ///
    pub fn accumulate_match_result<In>(
        &mut self,
        free_matching_lots_in: In::MatchingLots,
        locked_matching_lots_out: <In::Opposite as LegMarker>::MatchingLots,
    ) -> Result<(), GoblinError>
    where
        In: LegMarker + PairAccessor<MakerDelta<Base>, MakerDelta<Quote>, Result = MakerDelta<In>>,
    {
        let deltas_for_side = In::get_leg_mut(self);

        deltas_for_side.free_matching_lots_in += free_matching_lots_in;
        deltas_for_side.locked_matching_lots_out += locked_matching_lots_out;

        Ok(())
    }
}
