///! LegMarker for input base, also known as ask / sell
use crate::axis::leg::leg_math::LegMath;
use crate::axis::leg::{Base, Quote};
use crate::quantities::{BaseLots, BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, Ticks};

// Input Base = side Ask (sell)
impl LegMath for Base {
    type Opposite = Quote;

    type MatchingLots = BaseLots;

    fn matching_lots_taker(
        lots: Self::Lots,
        _base_lot_size: BaseLotsPerBaseUnit,
    ) -> Self::MatchingLots {
        lots
    }

    fn lots_taker(
        matching_lots: Self::MatchingLots,
        _base_lot_size: BaseLotsPerBaseUnit,
    ) -> Self::Lots {
        matching_lots
    }

    fn matching_lots_opposite(
        matching_lots: Self::MatchingLots,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> <Self::Opposite as LegMath>::MatchingLots {
        (tick_size * price) * matching_lots
    }

    fn matching_lots_maker(
        size: BaseLots,
        _tick_size: QuoteLotsPerBaseUnitPerTick,
        _price: Ticks,
    ) -> Self::MatchingLots {
        size
    }

    fn base_lots_maker(
        matching: Self::MatchingLots,
        _tick_size: QuoteLotsPerBaseUnitPerTick,
        _price: Ticks,
    ) -> BaseLots {
        matching
    }
}
