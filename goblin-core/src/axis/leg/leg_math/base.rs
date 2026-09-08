use crate::{
    axis::leg::{Base, LegMath, Quote},
    goblin_error::GoblinError,
    quantities::{BaseLots, BaseLotsPerBaseUnit, QuoteLotsPerBaseUnit},
};

// Input Base = side Ask (sell)
impl LegMath for Base {
    type Opposite = Quote;

    type MatchingLots = BaseLots;

    fn matching_lots_taker(
        lots: Self::Lots,
        _base_lot_size: BaseLotsPerBaseUnit,
    ) -> Result<Self::MatchingLots, GoblinError> {
        Ok(lots)
    }

    fn lots_taker(
        matching_lots: Self::MatchingLots,
        _base_lot_size: BaseLotsPerBaseUnit,
    ) -> Self::Lots {
        matching_lots
    }

    fn matching_lots_maker(
        base_lots: BaseLots,
        _price_in_quote_lots: QuoteLotsPerBaseUnit,
    ) -> Result<Self::MatchingLots, GoblinError> {
        Ok(base_lots)
    }

    fn base_lots_maker(
        matching_lots: Self::MatchingLots,
        _price_in_quote_lots: QuoteLotsPerBaseUnit,
    ) -> BaseLots {
        matching_lots
    }
}
