use crate::{settlement::market_delta::MakerDelta, types::LegMarker};

pub struct MatchResult<In: LegMarker> {
    pub maker_side_delta: MakerDelta<In>,
    pub released_by_self_trade: <In::Opposite as LegMarker>::MatchingLots,
}

impl<In: LegMarker> Default for MatchResult<In> {
    fn default() -> Self {
        Self {
            maker_side_delta: MakerDelta::default(),
            released_by_self_trade: <In::Opposite as LegMarker>::MatchingLots::default(),
        }
    }
}
