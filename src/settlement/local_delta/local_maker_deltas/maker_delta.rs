use crate::{axis::leg::leg_matcher::LegMatcher, settlement::MatchedLots};

/// Maker delta for taker side In
///
/// # Convention
///
/// * `In` represents the taker side. Maker side is opposite of `In`.
/// * If In = Base, then the maker fills the side Quote.
///
#[derive(Default, Clone, Copy, PartialEq)]
pub struct MakerDelta<In: LegMatcher> {
    /// Matched lots
    pub matched_lots: MatchedLots<In>,
}

impl<In: LegMatcher> MakerDelta<In> {
    pub const fn zero() -> Self {
        Self {
            matched_lots: MatchedLots::<In>::zero(),
        }
    }

    pub fn not_empty(&self) -> bool {
        *self != Self::zero()
    }
}
