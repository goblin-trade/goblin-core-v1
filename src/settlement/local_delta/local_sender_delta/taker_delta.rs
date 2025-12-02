use crate::quantities::QuantityOps;
use crate::types::LegMarker;

pub struct TakerDelta<In: LegMarker> {
    /// Input token traded in
    pub taker_in: In::MatchingLots,

    /// Output token obtained
    pub taker_out: <In::Opposite as LegMarker>::MatchingLots,

    /// Output token released due to taker self-trading
    pub taker_self_trade_unlocked: <In::Opposite as LegMarker>::MatchingLots,
}

impl<In: LegMarker> TakerDelta<In> {
    pub const fn new() -> Self {
        Self {
            taker_in: In::MatchingLots::ZERO,
            taker_out: <In::Opposite as LegMarker>::MatchingLots::ZERO,
            taker_self_trade_unlocked: <In::Opposite as LegMarker>::MatchingLots::ZERO,
        }
    }
}
