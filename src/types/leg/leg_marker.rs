use crate::{
    quantities::{
        AdjustedQuoteLots, BaseLots, BaseLotsPerBaseUnit, QuantityOps, QuoteLotsPerBaseUnitPerTick,
        Ticks,
    },
    types::LegQuantities,
};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct Base;

#[derive(Default, Clone, Copy, PartialEq)]
pub struct Quote;

pub trait LegMarker: Default + Clone + Copy + PartialEq + LegQuantities {
    type Opposite: LegMarker<Opposite = Self>;

    // The intermediary unit used for matching
    //
    // For any match, MatchingLots is transferred in and Opposite::MatchingLots
    // is obtained out
    // * Base in (Ask) case- MatchingLots = BaseLots, Opposite::MatchingLots = AdjustedQuoteLots
    // * Quote in (Bid) case- MatchingLots = AdjustedQuoteLots, Opposite::MatchingLots = BaseLots
    //
    // Use Self::MatchingLots to track amount consumed and Opposite::MatchingLots to get the output
    type MatchingLots: QuantityOps + PartialOrd;

    // Obtain MatchingLots from taker amount in
    fn matching_lots_taker(
        input_lots: Self::Lots,
        base_lot_size: BaseLotsPerBaseUnit,
    ) -> Self::MatchingLots;

    // Obtain MatchingLots from a resting order
    fn matching_lots_maker(
        size: BaseLots,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> Self::MatchingLots;

    // Decode MatchingLots into Lots
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

    fn base_lots_from_matching(
        matching_lots: Self::MatchingLots,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> BaseLots;

    /// Whether price_0 is closer to centre than price_1
    fn closer_to_centre(price_0: Ticks, price_1: Ticks) -> bool;
}

// Input Base = side Ask (sell)
impl LegMarker for Base {
    type Opposite = Quote;

    type MatchingLots = BaseLots;

    fn matching_lots_taker(
        input_lots: Self::Lots,
        _base_lot_size: BaseLotsPerBaseUnit,
    ) -> Self::MatchingLots {
        input_lots
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

    fn closer_to_centre(price_0: Ticks, price_1: Ticks) -> bool {
        price_0 < price_1
    }
}

// Input Quote = side Bid (buy)
impl LegMarker for Quote {
    type Opposite = Base;

    type MatchingLots = AdjustedQuoteLots;

    fn matching_lots_taker(
        input_lots: Self::Lots,
        base_lot_size: BaseLotsPerBaseUnit,
    ) -> Self::MatchingLots {
        input_lots * base_lot_size
    }

    fn matching_lots_maker(
        size: BaseLots,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> Self::MatchingLots {
        (tick_size * price) * size
    }

    fn decode_matching_lots(
        matching_lots: Self::MatchingLots,
        base_lot_size: BaseLotsPerBaseUnit,
    ) -> Self::Lots {
        matching_lots / base_lot_size
    }

    fn base_lots_from_matching(
        matching_lots: Self::MatchingLots,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> BaseLots {
        matching_lots / (tick_size * price)
    }

    fn closer_to_centre(price_0: Ticks, price_1: Ticks) -> bool {
        price_0 > price_1
    }
}
