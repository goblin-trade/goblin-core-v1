use core::ops::{Add, Sub};

use crate::{
    goblin_error::GoblinError,
    matching::MatchResult,
    quantities::{BaseLotsDelta, QuoteLotsDelta},
    types::{Base, LegMarker, Quote, SideMarker},
};

// #[derive(Default)]
// pub struct LegLotsDelta<L: LegMarker> {
//     pub consumed: L::LotsDelta,
//     pub locked: L::LotsDelta,
// }

// /// Consumed and locked deltas of msg.sender for a market
// #[derive(Default)]
// pub struct MarketLotsDelta {
//     pub base: LegLotsDelta<Base>,
//     pub quote: LegLotsDelta<Quote>,
// }

// /// Consumed and locked deltas of msg.sender for a market
// #[derive(Default)]
// pub struct MarketLotsDelta {
//     pub base_lots_consumed: BaseLotsDelta,
//     pub quote_lots_consumed: QuoteLotsDelta,
//     pub base_lots_locked: BaseLotsDelta,
//     pub quote_lots_locked: QuoteLotsDelta,
// }

// impl MarketLotsDelta {
//     /// Apply the match result to the market lots delta
//     ///
//     /// As per convention, we add when tokens are consumed by the engine and subtact
//     /// when tokens are emitted out.
//     ///
//     /// * lots_in are consumed by the engine, therefore add.
//     /// * lots_out are released by engine therefore subtract.
//     /// * self trade results in release of locked opposite tokens, therfore subtract.
//     pub fn apply_match_result<S: SideMarker>(
//         &mut self,
//         match_result: &MatchResult<S>,
//     ) -> Result<(), GoblinError> {
//         *S::consumed_for_side(self) =
//             S::consumed_for_side(self).add(match_result.pending_update.free_lots_in)?;
//         *S::Opposite::consumed_for_side(self) = S::Opposite::consumed_for_side(self)
//             .sub(match_result.pending_update.locked_lots_out)?;

//         *S::Opposite::locked_for_side(self) =
//             S::Opposite::locked_for_side(self).sub(match_result.released_by_self_trade)?;

//         Ok(())
//     }
// }
