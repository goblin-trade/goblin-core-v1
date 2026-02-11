use crate::axis::leg::leg_matcher::LegMatcher;
use crate::quantities::QuantityOps;
use crate::settlement::MatchedLots;

#[derive(Clone, Copy)]
pub struct TakerDelta<In: LegMatcher> {
    /// Matched lots
    pub matched_lots: MatchedLots<In>,

    /// Output token released due to taker self-trading
    pub taker_self_trade_unlocked: <In::Opposite as LegMatcher>::MatchingLots,
}

impl<In: LegMatcher> TakerDelta<In> {
    pub const fn zero() -> Self {
        Self {
            matched_lots: MatchedLots::<In>::zero(),
            taker_self_trade_unlocked: <In::Opposite as LegMatcher>::MatchingLots::ZERO,
        }
    }
}
