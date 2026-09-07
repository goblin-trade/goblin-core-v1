use crate::{
    axis::leg::{LegMatcher, LegQuantities},
    quantities::{BaseLots, BaseLotsPerBaseUnit, QuantityOps, QuoteLotsPerBaseUnit},
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
        price_in_quote_lots: QuoteLotsPerBaseUnit,
    ) -> Self::MatchingLots;

    /// Reciprocal of maker function
    fn base_lots_maker(
        matching_lots: Self::MatchingLots,
        price_in_quote_lots: QuoteLotsPerBaseUnit,
    ) -> BaseLots;
}
