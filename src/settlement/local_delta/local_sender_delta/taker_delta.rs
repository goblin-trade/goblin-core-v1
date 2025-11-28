use crate::types::LegMarker;

#[derive(Default, PartialEq)]
pub struct TakerDelta<In: LegMarker> {
    /// Input token traded in
    pub taker_in: In::MatchingLots,

    /// Output token obtained
    pub taker_out: <In::Opposite as LegMarker>::MatchingLots,

    /// Output token released due to taker self-trading
    pub taker_self_trade_unlocked: <In::Opposite as LegMarker>::MatchingLots,
}
