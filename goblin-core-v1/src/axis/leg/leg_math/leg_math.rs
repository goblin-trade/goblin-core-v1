use crate::{
    axis::leg::{leg_matcher::LegMatcher, leg_quantities::LegQuantities},
    quantities::{BaseLots, BaseLotsPerBaseUnit, QuantityOps, QuoteLotsPerBaseUnitPerTick, Ticks},
};

pub trait LegMath: LegQuantities {
    /// The opposite side. Opposite of opposite is Self.
    ///
    /// We can LegMatcher directly without circular dependency issues.
    type Opposite: LegMatcher<Opposite = Self>;

    /// The intermediary unit used for matching
    ///
    /// For any match, MatchingLots is transferred in and Opposite::MatchingLots
    /// is obtained out
    /// * Base in (Ask) case- MatchingLots = BaseLots, Opposite::MatchingLots = AdjustedQuoteLots
    /// * Quote in (Bid) case- MatchingLots = AdjustedQuoteLots, Opposite::MatchingLots = BaseLots
    ///
    /// Use Self::MatchingLots to track amount consumed and Opposite::MatchingLots to get the output
    type MatchingLots: QuantityOps;

    // TODO group functions based on if they take MatchingLots as input or output

    /// Obtain MatchingLots from taker amount in
    fn matching_lots_in(
        input_lots: Self::Lots,
        base_lot_size: BaseLotsPerBaseUnit,
    ) -> Self::MatchingLots;

    fn matching_lots_out(
        matching_lots: Self::MatchingLots,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> <Self::Opposite as LegMath>::MatchingLots;

    /// Decode MatchingLots into Lots
    fn decode_matching_lots(
        matching_lots: Self::MatchingLots,
        base_lot_size: BaseLotsPerBaseUnit,
    ) -> Self::Lots;

    // ------------

    /// Obtain MatchingLots from a resting order
    /// TODO replace with direct function opposite_matching_lots()
    fn matching_lots_maker(
        size: BaseLots,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> Self::MatchingLots;

    /// TODO replace with direct function opposite_matching_lots()
    fn base_lots_from_matching(
        matching_lots: Self::MatchingLots,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> BaseLots;

    fn opposite_lots_consumed_on_make(
        base_lots: BaseLots,
        base_lot_size: BaseLotsPerBaseUnit,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> <Self::Opposite as LegQuantities>::Lots {
        let matching_lots =
            <Self::Opposite as LegMath>::matching_lots_maker(base_lots, tick_size, price);
        <Self::Opposite as LegMath>::decode_matching_lots(matching_lots, base_lot_size)
    }
}
