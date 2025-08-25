use core::ops::{Add, Sub};

use crate::{
    define_custom_types, define_delta_operations, define_inter_type_operations,
    goblin_error::GoblinError,
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

/// Consumed and locked deltas for a market
#[derive(Default)]
pub struct MarketLotsDelta {
    pub base_lots_consumed: BaseLotsDelta,
    pub quote_lots_consumed: QuoteLotsDelta,
    pub base_lots_locked: BaseLotsDelta,
    pub quote_lots_locked: QuoteLotsDelta,
}

impl MarketLotsDelta {
    /// Apply the match result to the market lots delta
    ///
    /// As per convention, we add when tokens are consumed by the engine and subtact
    /// when tokens are emitted out.
    ///
    /// * lots_in are consumed by the engine, therefore add.
    /// * lots_out are released by engine therefore subtract.
    /// * self trade results in release of locked opposite tokens, therfore subtract.
    pub fn apply_match_result<S: SideMarker>(
        &mut self,
        match_result: &MatchResult<S>,
    ) -> Result<(), GoblinError> {
        *S::consumed_for_side(self) = S::consumed_for_side(self).add(match_result.lots_in)?;
        *S::Opposite::consumed_for_side(self) =
            S::Opposite::consumed_for_side(self).sub(match_result.lots_out)?;

        *S::Opposite::locked_for_side(self) =
            S::Opposite::locked_for_side(self).sub(match_result.released_by_self_trade)?;

        Ok(())
    }
}
