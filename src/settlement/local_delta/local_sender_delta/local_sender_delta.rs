use crate::{
    matching::MatchResult,
    types::{Base, Pair, Quote},
};

/// The sender delta of local namespace
///
/// Values are denominated in MatchingLots. Convert it to TakerTokenUpdate
/// so it can be added to the global delta
#[derive(Default)]
pub struct LocalSenderDelta {
    /// The results of matching take orders
    pub take_result_pair: Pair<MatchResult<Base>, MatchResult<Quote>>,
}
