use crate::{
    markets::IndexedMarket,
    matching::MatchResult,
    quantities::{
        AdjustedQuoteLots, AtomsDelta, BaseAtoms, BaseLots, BaseLotsDelta, BaseLotsPerBaseUnit,
        MarketDelta, QuoteAtoms, QuoteLots, QuoteLotsDelta, QuoteLotsPerBaseUnitPerTick,
        QuoteLotsPerQuoteUnit, Ticks, BASE_ATOMS_PER_BASE_UNIT, QUOTE_ATOMS_PER_QUOTE_UNIT,
    },
    state::MarketState,
};

pub struct Bid;
pub struct Ask;

pub trait SideMarker {
    // The input lots for a take order of this side
    type Lots: PartialOrd + Default + From<u64>;

    type DeltaLots;

    // The unit of accounting used for matching
    type Quote: Copy
        + PartialOrd
        + From<u64>
        + core::ops::Add<Output = Self::Quote>
        + core::ops::Sub<Output = Self::Quote>
        + core::ops::AddAssign
        + core::ops::SubAssign;

    // The unit of lots per unit
    type LotSize;

    // The unit of atoms
    type Atoms;

    // The opposite side
    type Opposite: SideMarker;

    // Default price limit for take orders
    const DEFAULT_PRICE_LIMIT: Ticks;

    fn price_limit_valid(price_limit: Ticks) -> bool;

    fn get_resting_order_quote(
        size: BaseLots,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> Self::Quote;

    fn get_opposite_quote(
        quote: Self::Quote,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> <Self::Opposite as SideMarker>::Quote;

    fn get_budget(num_lots: Self::Lots, base_lot_size: BaseLotsPerBaseUnit) -> Self::Quote;

    fn get_lots_from_quote(quote: Self::Quote, base_lot_size: BaseLotsPerBaseUnit) -> Self::Lots;

    /// Whether price_1 is closer to centre than price_0
    fn closer_to_centre(price_0: Ticks, price_1: Ticks) -> bool;

    fn best_price_mut(market_state: &mut MarketState) -> &mut Ticks;

    fn get_lot_size(indexed_market: &IndexedMarket) -> Self::LotSize;

    fn lots_delta_to_atoms_delta(lots: Self::DeltaLots, lot_size: Self::LotSize) -> AtomsDelta;

    fn delta_for_side(market_delta: &mut MarketDelta) -> &mut Self::DeltaLots;
}

impl SideMarker for Bid {
    type Lots = QuoteLots;
    type DeltaLots = QuoteLotsDelta;
    type Quote = AdjustedQuoteLots;
    type LotSize = QuoteLotsPerQuoteUnit;
    type Atoms = QuoteAtoms;
    type Opposite = Ask;

    const DEFAULT_PRICE_LIMIT: Ticks = Ticks::MAX;

    fn price_limit_valid(price_limit: Ticks) -> bool {
        price_limit > Ticks::ZERO
    }

    fn get_resting_order_quote(
        size: BaseLots,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> Self::Quote {
        (tick_size * price) * size
    }

    fn get_opposite_quote(
        quote: Self::Quote,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> <Self::Opposite as SideMarker>::Quote {
        quote / (tick_size * price)
    }

    fn get_budget(num_lots: Self::Lots, base_lot_size: BaseLotsPerBaseUnit) -> Self::Quote {
        num_lots * base_lot_size
    }

    fn get_lots_from_quote(quote: Self::Quote, base_lot_size: BaseLotsPerBaseUnit) -> Self::Lots {
        quote / base_lot_size
    }

    fn closer_to_centre(price_0: Ticks, price_1: Ticks) -> bool {
        price_1 > price_0
    }

    fn best_price_mut(market_state: &mut MarketState) -> &mut Ticks {
        &mut market_state.best_bid_price
    }

    fn get_lot_size(indexed_market: &IndexedMarket) -> Self::LotSize {
        indexed_market.quote_lot_size
    }

    fn lots_delta_to_atoms_delta(lots: Self::DeltaLots, lot_size: Self::LotSize) -> AtomsDelta {
        (QUOTE_ATOMS_PER_QUOTE_UNIT / lot_size) * lots
    }

    fn delta_for_side(market_delta: &mut MarketDelta) -> &mut Self::DeltaLots {
        &mut market_delta.quote_lots_delta
    }
}

impl SideMarker for Ask {
    type Lots = BaseLots;
    type DeltaLots = BaseLotsDelta;
    type Quote = BaseLots;
    type LotSize = BaseLotsPerBaseUnit;
    type Atoms = BaseAtoms;
    type Opposite = Bid;

    const DEFAULT_PRICE_LIMIT: Ticks = Ticks::ZERO;

    fn price_limit_valid(_price_limit: Ticks) -> bool {
        true
    }

    fn get_resting_order_quote(
        size: BaseLots,
        _tick_size: QuoteLotsPerBaseUnitPerTick,
        _price: Ticks,
    ) -> Self::Quote {
        size
    }

    fn get_opposite_quote(
        quote: Self::Quote,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> <Self::Opposite as SideMarker>::Quote {
        (tick_size * price) * quote
    }

    fn get_budget(num_lots: Self::Lots, _base_lot_size: BaseLotsPerBaseUnit) -> Self::Quote {
        num_lots
    }

    fn get_lots_from_quote(quote: Self::Quote, _base_lot_size: BaseLotsPerBaseUnit) -> Self::Lots {
        quote
    }

    fn closer_to_centre(price_0: Ticks, price_1: Ticks) -> bool {
        price_1 < price_0
    }

    fn best_price_mut(market_state: &mut MarketState) -> &mut Ticks {
        &mut market_state.best_ask_price
    }

    fn get_lot_size(indexed_market: &IndexedMarket) -> Self::LotSize {
        indexed_market.base_lot_size
    }

    fn lots_delta_to_atoms_delta(lots: Self::DeltaLots, lot_size: Self::LotSize) -> AtomsDelta {
        (BASE_ATOMS_PER_BASE_UNIT / lot_size) * lots
    }

    fn delta_for_side(market_delta: &mut MarketDelta) -> &mut Self::DeltaLots {
        &mut market_delta.base_lots_delta
    }
}
