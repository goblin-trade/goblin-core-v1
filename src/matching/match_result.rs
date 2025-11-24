use crate::types::LegMarker;

#[derive(Default, PartialEq)]
pub struct MatchResult<In: LegMarker> {
    /// Lots of input token traded in on match
    pub free_matching_lots_in: In::MatchingLots,

    /// Lots of output token obtained on match
    pub locked_matching_lots_out: <In::Opposite as LegMarker>::MatchingLots,

    /// Lots of output token released on self trade
    pub released_by_self_trade: <In::Opposite as LegMarker>::MatchingLots,
}
