///! LegMarker for input quote, also known as bid / buy
use crate::{
    axis::leg::{leg_math::LegMath, Base, Quote},
    quantities::{
        AdjustedQuoteLots, BaseLots, BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, Ticks,
    },
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
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> Self::MatchingLots {
        (tick_size * price) * base_lots
    }

    fn base_lots_maker(
        matching_lots: Self::MatchingLots,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> BaseLots {
        matching_lots / (tick_size * price)
    }
}
