use crate::{
    goblin_error::GoblinError,
    settlement::local_delta::MakerDelta,
    types::{Base, LegMarker, Pair, PairAccessor, Quote},
};

/// Maker deltas for base and quote sides
pub type MakerDeltaPair = Pair<MakerDelta<Base>, MakerDelta<Quote>>;

impl MakerDeltaPair {
    pub const fn new() -> Self {
        Self {
            base: MakerDelta::<Base>::new(),
            quote: MakerDelta::<Quote>::new(),
        }
    }

    /// Accumulate the matched lots for a given maker on a given side.
    ///
    /// # Overflow
    /// - `free_lots_in` (credited lots) may overflow; this only affects
    ///   the maker’s credited balance and does not harm solvency.
    /// - `locked_lots_out` (debited lots) must not overflow, but is safe
    ///   since resting orders are always backed by reserves.
    ///
    pub fn accumulate_match_result<In>(
        &mut self,
        taker_in: In::MatchingLots,
        taker_out: <In::Opposite as LegMarker>::MatchingLots,
    ) -> Result<(), GoblinError>
    where
        In: LegMarker + PairAccessor<MakerDelta<Base>, MakerDelta<Quote>, Result = MakerDelta<In>>,
    {
        let deltas_for_side = In::get_leg_mut(self);

        deltas_for_side.taker_in += taker_in;
        deltas_for_side.taker_out += taker_out;

        Ok(())
    }
}
