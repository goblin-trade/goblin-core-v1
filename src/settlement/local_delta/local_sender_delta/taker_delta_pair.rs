use crate::{
    settlement::local_delta::TakerDelta,
    types::{Base, LegMarker, Pair, PairAccessor, Quote},
};

/// The results of matching take orders
pub type TakerDeltaPair = Pair<TakerDelta<Base>, TakerDelta<Quote>>;

impl TakerDeltaPair {
    pub const fn new() -> Self {
        Self {
            base: TakerDelta::<Base>::new(),
            quote: TakerDelta::<Quote>::new(),
        }
    }

    /// Set the result after matching
    ///
    /// Implementation differs from MakerDeltaPair::accumulate_match_result(), where
    /// we add results to instead of setting from zero.
    pub fn set_match_result<In>(
        &mut self,
        taker_in: In::MatchingLots,
        taker_out: <In::Opposite as LegMarker>::MatchingLots,
        taker_self_trade_unlocked: <In::Opposite as LegMarker>::MatchingLots,
    ) where
        In: LegMarker + PairAccessor<TakerDelta<Base>, TakerDelta<Quote>, Result = TakerDelta<In>>,
    {
        let deltas_for_side = In::get_leg_mut(self);
        *deltas_for_side = TakerDelta {
            taker_in,
            taker_out,
            taker_self_trade_unlocked,
        };
    }
}
