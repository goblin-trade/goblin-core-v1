use crate::quantities::QuantityOps;
use crate::settlement::MatchedLots;
use crate::types::LegMarker;

#[derive(Clone, Copy)]
pub struct TakerDelta<In: LegMarker> {
    /// Matched lots
    pub matched_lots: MatchedLots<In>,

    /// Output token released due to taker self-trading
    pub taker_self_trade_unlocked: <In::Opposite as LegMarker>::MatchingLots,
}

impl<In: LegMarker> TakerDelta<In> {
    pub const fn zero() -> Self {
        Self {
            matched_lots: MatchedLots::<In>::zero(),
            taker_self_trade_unlocked: <In::Opposite as LegMarker>::MatchingLots::ZERO,
        }
    }
}
