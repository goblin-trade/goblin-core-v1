use crate::{
    axis::leg::{
        leg_constants::LegConstants, leg_coordinates::LegCoordinates, leg_iterator::LegIterator,
        leg_quantities::LegQuantities, leg_reader::LegReader, leg_validator::LegValidator, Base,
        Leg, Quote,
    },
    matching::bitmap::outer_bitmap_index::OuterBitmapIndex,
    quantities::{
        BaseLots, BaseLotsPerBaseUnit, QuantityOps, QuoteLots, QuoteLotsPerBaseUnitPerTick, Ticks,
    },
    settlement::MatchedLots,
    state::MarketState,
    types::{StoreReader, Tuple},
};

/// Conversions for matching orders
pub trait LegMatcher:
    LegQuantities
    + LegConstants
    + LegValidator
    + LegCoordinates
    + LegIterator
    + LegReader
    + StoreReader<Tuple<MatchedLots<Base>, MatchedLots<Quote>, Leg>, Result = MatchedLots<Self>>
    + StoreReader<Tuple<QuoteLots, BaseLots, Leg>, Result = <Self::Opposite as LegQuantities>::Lots>
{
    /// The opposite side
    /// Opposite of opposite is Self
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
    fn matching_lots_in(
        input_lots: Self::Lots,
        base_lot_size: BaseLotsPerBaseUnit,
    ) -> Self::MatchingLots;

    fn matching_lots_out(
        matching_lots: Self::MatchingLots,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> <Self::Opposite as LegMatcher>::MatchingLots;

    /// Decode MatchingLots into Lots
    fn decode_matching_lots(
        matching_lots: Self::MatchingLots,
        base_lot_size: BaseLotsPerBaseUnit,
    ) -> Self::Lots;

    fn matching_lots_to_atoms(
        matching_lots: Self::MatchingLots,
        base_lot_size: BaseLotsPerBaseUnit,
        atoms_per_lot: Self::AtomsPerLot,
    ) -> Self::Atoms {
        let lots = Self::decode_matching_lots(matching_lots, base_lot_size);
        let atoms: Self::Atoms = lots * atoms_per_lot;

        atoms
    }

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

    /// Maker functions

    fn maker_deposit(
        base_lots: BaseLots,
        base_lot_size: BaseLotsPerBaseUnit,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> <Self::Opposite as LegQuantities>::Lots;

    // fn valid_open_price(market_state: &MarketState, price: Ticks) -> bool {
    //     let opposite_price = Self::Opposite::get(&market_state.last_coordinates).price;
    //     Self::closer_to_opposite_pole(opposite_price, price)
    // }
}
