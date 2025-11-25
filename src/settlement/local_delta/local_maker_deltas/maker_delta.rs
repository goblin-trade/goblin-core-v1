use crate::types::LegMarker;

/// Maker delta for taker side In
///
/// # Convention
///
/// * `In` represents the taker side. Maker side is opposite of `In`.
/// * If In = Base, then the maker fills the side Quote.
///
#[derive(Default, Clone, Copy)]
pub struct MakerDelta<In: LegMarker> {
    pub free_matching_lots_in: In::MatchingLots,
    pub locked_matching_lots_out: <In::Opposite as LegMarker>::MatchingLots,
}
