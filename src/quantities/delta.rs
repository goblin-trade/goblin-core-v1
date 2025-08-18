use crate::{
    define_custom_types, define_delta_operations, define_inter_type_operations,
    matching::MatchResult,
    quantities::{BaseAtomsPerBaseLot, BaseLots, QuoteAtomsPerQuoteLot, QuoteLots},
    types::SideMarker,
};

use super::Atoms;

// Delta for atoms
define_custom_types!(AtomsDelta<i64>);
define_delta_operations!(AtomsDelta<i64>, Atoms<u64>);

// Delta for lots
define_custom_types!(BaseLotsDelta<i64>, QuoteLotsDelta<i64>);
define_delta_operations!(BaseLotsDelta<i64>, BaseLots<u64>);
define_delta_operations!(QuoteLotsDelta<i64>, QuoteLots<u64>);

define_inter_type_operations!(
    BaseAtomsPerBaseLot<u64>,
    BaseLotsDelta<i64>,
    AtomsDelta<i64>
);
define_inter_type_operations!(
    QuoteAtomsPerQuoteLot<u64>,
    QuoteLotsDelta<i64>,
    AtomsDelta<i64>
);

impl AtomsDelta {
    pub fn abs(&self) -> Atoms {
        Atoms(self.0.abs() as u64)
    }
}

#[derive(Default)]
pub struct MarketDelta {
    pub base_lots_delta: BaseLotsDelta,
    pub quote_lots_delta: QuoteLotsDelta,
}

impl MarketDelta {
    pub fn apply_match<S: SideMarker>(&mut self, match_result: &MatchResult<S>) {
        let delta = S::delta_for_side(self);

        // *delta = delta.add(match_result.lots_in)?;

        // *S::delta_for_side(self) = S::delta_for_side(self).add(match_result.lots_in);
    }
}
