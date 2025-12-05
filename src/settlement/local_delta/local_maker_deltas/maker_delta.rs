use crate::settlement::MatchedLots;
use crate::types::LegMarker;

/// Maker delta for taker side In
///
/// # Convention
///
/// * `In` represents the taker side. Maker side is opposite of `In`.
/// * If In = Base, then the maker fills the side Quote.
///
#[derive(Default, Clone, Copy, PartialEq)]
pub struct MakerDelta<In: LegMarker> {
    /// Matched lots
    pub matched_lots: MatchedLots<In>,
}

impl<In: LegMarker> MakerDelta<In> {
    pub const fn new() -> Self {
        Self {
            matched_lots: MatchedLots::<In>::new(),
        }
    }

    pub fn not_empty(&self) -> bool {
        *self != Self::new()
    }
}
