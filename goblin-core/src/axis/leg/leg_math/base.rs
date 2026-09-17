use crate::{
    axis::leg::{Base, LegMath, Quote},
    goblin_error::GoblinError,
    quantities::{BaseLots, BaseLotsPerBaseUnit, QuoteLotsPerBaseUnit},
};

// Input Base = side Ask (sell)
impl LegMath for Base {
    type Opposite = Quote;

    type MatchingLots = BaseLots<u64>;

    fn matching_lots_taker(
        lots: Self::Lots,
        _base_lot_size: BaseLotsPerBaseUnit<u64>,
    ) -> Result<Self::MatchingLots, GoblinError> {
        Ok(lots)
    }

    fn lots_taker(
        matching_lots: Self::MatchingLots,
        _base_lot_size: BaseLotsPerBaseUnit<u64>,
    ) -> Self::Lots {
        matching_lots
    }

    fn matching_lots_maker(
        base_lots: BaseLots<u64>,
        _price_in_quote_lots: QuoteLotsPerBaseUnit<u64>,
    ) -> Result<Self::MatchingLots, GoblinError> {
        Ok(base_lots)
    }

    fn base_lots_maker(
        matching_lots: Self::MatchingLots,
        _price_in_quote_lots: QuoteLotsPerBaseUnit<u64>,
    ) -> BaseLots<u64> {
        matching_lots
    }
}
