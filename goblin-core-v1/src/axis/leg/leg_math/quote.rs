///! LegMarker for input quote, also known as bid / buy
use crate::{
    axis::leg::{leg_math::LegMath, Base, Quote},
    quantities::{AdjustedQuoteLots, BaseLots, BaseLotsPerBaseUnit, QuoteLotsPerBaseUnit},
};

impl LegMath for Quote {
    type Opposite = Base;

    type MatchingLots = AdjustedQuoteLots;

    fn matching_lots_taker(
        lots: Self::Lots,
        base_lot_size: BaseLotsPerBaseUnit,
    ) -> Self::MatchingLots {
        lots * base_lot_size
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
    ) -> Self::MatchingLots {
        price_in_quote_lots * base_lots
    }

    fn base_lots_maker(
        matching_lots: Self::MatchingLots,
        price_in_quote_lots: QuoteLotsPerBaseUnit,
    ) -> BaseLots {
        matching_lots / price_in_quote_lots
    }
}
