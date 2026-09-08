use crate::{
    axis::leg::{Base, Quote, leg_math::LegMath},
    goblin_error::GoblinError,
    quantities::{AdjustedQuoteLots, BaseLots, BaseLotsPerBaseUnit, QuoteLotsPerBaseUnit},
};

impl LegMath for Quote {
    type Opposite = Base;

    type MatchingLots = AdjustedQuoteLots;

    fn matching_lots_taker(
        lots: Self::Lots,
        base_lot_size: BaseLotsPerBaseUnit,
    ) -> Result<Self::MatchingLots, GoblinError> {
        lots.checked_mul(base_lot_size).ok_or(GoblinError::Overflow)
    }

    fn lots_taker(
        matching_lots: Self::MatchingLots,
        base_lot_size: BaseLotsPerBaseUnit,
    ) -> Self::Lots {
        matching_lots / base_lot_size
    }

    fn matching_lots_maker(
        base_lots: BaseLots,
        price_in_quote_lots: QuoteLotsPerBaseUnit,
    ) -> Result<Self::MatchingLots, GoblinError> {
        price_in_quote_lots
            .checked_mul(base_lots)
            .ok_or(GoblinError::Overflow)
    }

    fn base_lots_maker(
        matching_lots: Self::MatchingLots,
        price_in_quote_lots: QuoteLotsPerBaseUnit,
    ) -> BaseLots {
        matching_lots / price_in_quote_lots
    }
}
