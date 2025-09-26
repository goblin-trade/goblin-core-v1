use crate::{
    markets::{IndexedMarket, MarketLeg},
    matching::SenderSideDelta,
    quantities::{
        AdjustedQuoteLots, Atoms, BaseAtoms, BaseAtomsPerBaseLot, BaseAtomsPerBaseUnit, BaseLots,
        BaseLotsPerBaseUnit, BaseUnits, QuantityOps, QuoteAtoms, QuoteAtomsPerQuoteLot,
        QuoteAtomsPerQuoteUnit, QuoteLots, QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit,
        QuoteUnits, Ticks, BASE_ATOMS_PER_BASE_UNIT, QUOTE_ATOMS_PER_QUOTE_UNIT,
    },
    settlement::{MakerDelta, MakerSideDelta, SenderDelta},
    state::MarketState,
    tokens::TokenIndex,
};
use core::ops::{Div, Mul, Rem};

pub struct LegPair<B, Q> {
    pub base: B,
    pub quote: Q,
}

// Scalar
type TokenIndexPair = LegPair<TokenIndex, TokenIndex>;

// Struct with generic
type MarketPair = LegPair<MarketLeg<Base>, MarketLeg<Quote>>;

#[derive(Default, Clone, Copy)]
pub struct Base;

#[derive(Default, Clone, Copy)]
pub struct Quote;

pub trait LegMarker {
    type Opposite: LegMarker<Opposite = Self>;

    type Limb<B, Q>;
    fn get_on_pair<'a, B, Q>(pair: &'a LegPair<B, Q>) -> &'a Self::Limb<B, Q>;
    fn get_on_pair_mut<'a, B, Q>(pair: &'a mut LegPair<B, Q>) -> &'a mut Self::Limb<B, Q>;
    fn take_from_pair<B, Q>(pair: LegPair<B, Q>) -> Self::Limb<B, Q>;

    // Basic quantities
    type Lots: QuantityOps + From<u64> + PartialOrd + Mul<Self::AtomsPerLot, Output = Self::Atoms>;
    type Units: QuantityOps;
    type Atoms: QuantityOps + Into<Atoms>;

    // Ratios
    type LotsPerUnit: QuantityOps;
    type AtomsPerUnit: QuantityOps
        + Rem<Self::LotsPerUnit, Output = Self::AtomsPerUnit>
        + Div<Self::LotsPerUnit, Output = Self::AtomsPerLot>;
    type AtomsPerLot: QuantityOps;

    const ATOMS_PER_UNIT: Self::AtomsPerUnit;

    /// Ensure that market has an integer number of atoms per lot
    ///
    /// As ATOMS_PER_UNIT is hardcoded to 10^6 for base and quote, this is effectively
    ///
    ///  **10^6 % Lot size == 0**
    ///
    /// lots_per_unit is also called lot_size
    fn lots_per_unit_valid(lots_per_unit: Self::LotsPerUnit) -> bool {
        Self::ATOMS_PER_UNIT % lots_per_unit == Self::AtomsPerUnit::ZERO
    }

    /// The number of atoms per lot
    ///
    /// Since we have validated the modulo invariant, this will give a whole number
    fn atoms_per_lot(lots_per_unit: Self::LotsPerUnit) -> Self::AtomsPerLot {
        Self::ATOMS_PER_UNIT / lots_per_unit
    }

    // Trade inputs
    const DEFAULT_PRICE_LIMIT: Ticks;

    fn price_limit_valid(_price_limit: Ticks) -> bool;

    fn maker_side_delta_ref(market_maker_delta: &MakerDelta) -> &MakerSideDelta<Self>
    where
        Self: Sized;

    fn maker_side_delta_mut<'a>(
        market_maker_delta: &'a mut MakerDelta,
    ) -> &'a mut MakerSideDelta<Self>
    where
        Self: Sized;

    fn sender_side_delta(sender_delta: &SenderDelta) -> &SenderSideDelta<Self>
    where
        Self: Sized;

    fn market_leg(indexed_market: &IndexedMarket) -> &MarketLeg<Self>
    where
        Self: Sized;

    // Match function

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
    ) -> Atoms {
        let lots = Self::decode_matching_lots(matching_lots, base_lot_size);
        (lots * atoms_per_lot).into()
    }

    fn base_lots_from_matching(
        matching_lots: Self::MatchingLots,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> BaseLots;

    fn best_market_price_mut(market_state: &mut MarketState) -> &mut Ticks;

    /// Whether price_0 is closer to centre than price_1
    fn closer_to_centre(price_0: Ticks, price_1: Ticks) -> bool;
}

// Input Base = side Ask (sell)
impl LegMarker for Base {
    type Opposite = Quote;

    type Limb<B, Q> = B;

    fn get_on_pair<'a, B, Q>(pair: &'a LegPair<B, Q>) -> &'a B {
        &pair.base
    }
    fn get_on_pair_mut<'a, B, Q>(pair: &'a mut LegPair<B, Q>) -> &'a mut B {
        &mut pair.base
    }
    fn take_from_pair<B, Q>(pair: LegPair<B, Q>) -> B {
        pair.base
    }

    type Lots = BaseLots;
    type Units = BaseUnits;
    type Atoms = BaseAtoms;

    type LotsPerUnit = BaseLotsPerBaseUnit;
    type AtomsPerUnit = BaseAtomsPerBaseUnit;
    type AtomsPerLot = BaseAtomsPerBaseLot;

    const ATOMS_PER_UNIT: Self::AtomsPerUnit = BASE_ATOMS_PER_BASE_UNIT;

    const DEFAULT_PRICE_LIMIT: Ticks = Ticks::ZERO;

    fn price_limit_valid(_price_limit: Ticks) -> bool {
        true
    }

    fn maker_side_delta_ref(market_maker_delta: &MakerDelta) -> &MakerSideDelta<Self> {
        &market_maker_delta.base_in
    }

    fn maker_side_delta_mut<'a>(
        market_maker_delta: &'a mut MakerDelta,
    ) -> &'a mut MakerSideDelta<Self> {
        &mut market_maker_delta.base_in
    }

    fn sender_side_delta(sender_delta: &SenderDelta) -> &SenderSideDelta<Self> {
        &sender_delta.take_base_in
    }

    fn market_leg(indexed_market: &IndexedMarket) -> &MarketLeg<Self> {
        &indexed_market.base
    }

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

    fn best_market_price_mut(market_state: &mut MarketState) -> &mut Ticks {
        &mut market_state.best_ask_price
    }

    fn closer_to_centre(price_0: Ticks, price_1: Ticks) -> bool {
        price_0 < price_1
    }
}

// Input Quote = side Bid (buy)
impl LegMarker for Quote {
    type Opposite = Base;

    type Limb<B, Q> = Q;

    fn get_on_pair<'a, B, Q>(pair: &'a LegPair<B, Q>) -> &'a Q {
        &pair.quote
    }
    fn get_on_pair_mut<'a, B, Q>(pair: &'a mut LegPair<B, Q>) -> &'a mut Q {
        &mut pair.quote
    }
    fn take_from_pair<B, Q>(pair: LegPair<B, Q>) -> Q {
        pair.quote
    }

    type Lots = QuoteLots;
    type Units = QuoteUnits;
    type Atoms = QuoteAtoms;

    type LotsPerUnit = QuoteLotsPerQuoteUnit;
    type AtomsPerUnit = QuoteAtomsPerQuoteUnit;
    type AtomsPerLot = QuoteAtomsPerQuoteLot;

    const ATOMS_PER_UNIT: Self::AtomsPerUnit = QUOTE_ATOMS_PER_QUOTE_UNIT;

    const DEFAULT_PRICE_LIMIT: Ticks = Ticks::MAX;

    fn price_limit_valid(price_limit: Ticks) -> bool {
        price_limit > Ticks::ZERO
    }

    fn maker_side_delta_ref(market_maker_delta: &MakerDelta) -> &MakerSideDelta<Self> {
        &market_maker_delta.quote_in
    }

    fn maker_side_delta_mut<'a>(
        market_maker_delta: &'a mut MakerDelta,
    ) -> &'a mut MakerSideDelta<Self> {
        &mut market_maker_delta.quote_in
    }

    fn sender_side_delta(sender_delta: &SenderDelta) -> &SenderSideDelta<Self> {
        &sender_delta.take_quote_in
    }

    fn market_leg(indexed_market: &IndexedMarket) -> &MarketLeg<Self> {
        &indexed_market.quote
    }

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

    fn best_market_price_mut(market_state: &mut MarketState) -> &mut Ticks {
        &mut market_state.best_bid_price
    }

    fn closer_to_centre(price_0: Ticks, price_1: Ticks) -> bool {
        price_0 > price_1
    }
}
