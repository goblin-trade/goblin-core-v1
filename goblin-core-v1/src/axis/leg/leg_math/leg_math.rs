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
    fn matching_lots_taker(
        lots: Self::Lots,
        base_lot_size: BaseLotsPerBaseUnit,
    ) -> Self::MatchingLots;

    /// Decode MatchingLots into Lots
    fn lots_taker(
        matching_lots: Self::MatchingLots,
        base_lot_size: BaseLotsPerBaseUnit,
    ) -> Self::Lots;

    /// Obtain MatchingLots from a resting order
    fn matching_lots_maker(
        base_lots: BaseLots,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> Self::MatchingLots;

    /// Reciprocal of maker function
    fn base_lots_maker(
        matching_lots: Self::MatchingLots,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> BaseLots;

    /// Steps to derive opposite values
    ///
    /// 1. MatchingLots to opposite lots:
    ///   - In::base_lots_maker() -> Opposite::matching_lots_maker() -> Opposite::lots_taker()
    ///
    /// 2. Base lots to opposite lots
    ///   - Opposite::matching_lots_maker() -> Opposite::lots_taker()
    ///

    /// The ones on top are the main relationships. Rest is derived

    fn matching_lots_opposite(
        matching_lots: Self::MatchingLots,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> <Self::Opposite as LegMath>::MatchingLots;
}
