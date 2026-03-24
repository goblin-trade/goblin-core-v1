use crate::axis::leg::leg_quantities::LegQuantities;
///! LegMarker for input base, also known as ask / sell
use crate::axis::leg::{leg_matcher::LegMatcher, Base, Quote};
use crate::quantities::{BaseLots, BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, Ticks};

// Input Base = side Ask (sell)
impl LegMatcher for Base {
    type Opposite = Quote;

    type MatchingLots = BaseLots;

    fn matching_lots_in(
        input_lots: Self::Lots,
        _base_lot_size: BaseLotsPerBaseUnit,
    ) -> Self::MatchingLots {
        input_lots
    }

    fn matching_lots_out(
        matching_lots: Self::MatchingLots,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> <Self::Opposite as LegMatcher>::MatchingLots {
        (tick_size * price) * matching_lots
    }

    fn matching_lots_maker(
        size: BaseLots,
        _tick_size: QuoteLotsPerBaseUnitPerTick,
        _price: Ticks,
    ) -> Self::MatchingLots {
        size
    }

    fn decode_matching_lots(
        matching_lots: Self::MatchingLots,
        _base_lot_size: BaseLotsPerBaseUnit,
    ) -> Self::Lots {
        matching_lots
    }

    fn base_lots_from_matching(
        matching: Self::MatchingLots,
        _tick_size: QuoteLotsPerBaseUnitPerTick,
        _price: Ticks,
    ) -> BaseLots {
        matching
    }

    fn maker_deposit(
        base_lots: BaseLots,
        base_lot_size: BaseLotsPerBaseUnit,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> <Self::Opposite as LegQuantities>::Lots {
        tick_size * price * base_lots / base_lot_size
    }
}
