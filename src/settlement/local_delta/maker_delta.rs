use crate::types::LegMarker;

/// Maker delta for taker side In
///
/// # Convention
///
/// * `In` represents the taker side. Maker side is opposite of `In`.
/// * If In = Base, then the maker fills the side Quote.
///
#[derive(Clone, Copy)]
pub struct MakerDelta<In: LegMarker> {
    pub free_matching_lots_in: In::MatchingLots,
    pub locked_matching_lots_out: <In::Opposite as LegMarker>::MatchingLots,
}

impl<In: LegMarker> Default for MakerDelta<In> {
    fn default() -> Self {
        Self {
            free_matching_lots_in: In::MatchingLots::default(),
            locked_matching_lots_out: <In::Opposite as LegMarker>::MatchingLots::default(),
        }
    }
}
