use core::ops::Add;

use crate::{
    goblin_error::GoblinError,
    markets::IndexedMarket,
    quantities::{
        AdjustedQuoteLots, Atoms, BaseAtoms, BaseAtomsPerBaseLot, BaseLots, BaseLotsPerBaseUnit,
        QuoteAtoms, QuoteAtomsPerQuoteLot, QuoteLots, QuoteLotsPerBaseUnitPerTick,
        QuoteLotsPerQuoteUnit, Ticks,
    },
    settlement::{MakerUpdate, MakerUpdateSide, PendingMakerStoreUpdates, PendingStoreKey},
    state::{MakerStore, MarketState},
    tokens::TokenIndex,
    types::{Address, LegMarker},
};

pub struct Bid;
pub struct Ask;

pub trait SideMarker {
    // The input lots for a take order of this side
    type Lots: Copy
        + PartialOrd
        + Default
        + core::ops::AddAssign
        + From<u64>
        + core::ops::Mul<Self::AtomsPerLot, Output = Self::Atoms>;

    // The unit of accounting used for matching
    type MatchingLots: Copy
        + PartialOrd
        + From<u64>
        + core::ops::Add<Output = Self::MatchingLots>
        + core::ops::Sub<Output = Self::MatchingLots>
        + core::ops::AddAssign
        + core::ops::SubAssign;

    // The unit of lots per unit
    type LotSize;

    // The unit of atoms
    type Atoms;

    type AtomsPerLot;

    // The opposite side
    type Opposite: SideMarker;

    // Default price limit for take orders
    const DEFAULT_PRICE_LIMIT: Ticks;

    fn price_limit_valid(price_limit: Ticks) -> bool;

    fn get_quote_from_base_lots(
        size: BaseLots,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> Self::MatchingLots;

    fn get_base_lots_from_quote(
        quote: Self::MatchingLots,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> BaseLots;

    // Redundant, TODO remove
    fn get_opposite_quote(
        quote: Self::MatchingLots,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> <Self::Opposite as SideMarker>::MatchingLots;

    fn get_budget(num_lots: Self::Lots, base_lot_size: BaseLotsPerBaseUnit) -> Self::MatchingLots;

    fn get_lots_from_quote(
        quote: Self::MatchingLots,
        base_lot_size: BaseLotsPerBaseUnit,
    ) -> Self::Lots;

    /// Whether price_1 is closer to centre than price_0
    fn closer_to_centre(price_0: Ticks, price_1: Ticks) -> bool;

    fn best_price_mut(market_state: &mut MarketState) -> &mut Ticks;

    fn get_lot_size(indexed_market: &IndexedMarket) -> Self::LotSize;

    fn atoms_per_lot(indexed_market: &IndexedMarket) -> Self::AtomsPerLot;

    fn consumed_for_side(market_delta: &mut MarketLotsDelta) -> &mut Self::DeltaLots;

    fn locked_for_side(market_delta: &mut MarketLotsDelta) -> &mut Self::DeltaLots;

    /// Update token stores for a maker upon a match
    ///
    /// # Arguments
    ///
    /// * `base_store`- The base store
    /// * `quote_store` - The quote stoore
    /// * `atoms`- The atoms lost by the taker. Add to the maker's free tokens.
    /// * `atoms_opposite`- The atoms gained by the maker. Subtract from maker's locked tokens.
    fn update_maker_stores(
        base_store: &mut impl MakerStore,
        quote_store: &mut impl MakerStore,
        atoms: Self::Atoms,
        atoms_opposite: <Self::Opposite as SideMarker>::Atoms,
    );

    fn maker_update_for_side_ref<'a>(maker_update: &'a MakerUpdate) -> &'a MakerUpdateSide<Self>
    where
        Self: Sized;

    fn maker_update_for_side_mut<'a>(
        maker_update: &'a mut MakerUpdate,
    ) -> &'a mut MakerUpdateSide<Self>
    where
        Self: Sized;

    // Token index of the input token, i.e. 'Lots'
    fn token_index_for_side(indexed_market: &IndexedMarket) -> TokenIndex;
}

impl SideMarker for Bid {
    type Lots = QuoteLots;
    type MatchingLots = AdjustedQuoteLots;
    type LotSize = QuoteLotsPerQuoteUnit;
    type Atoms = QuoteAtoms;
    type AtomsPerLot = QuoteAtomsPerQuoteLot;
    type Opposite = Ask;

    const DEFAULT_PRICE_LIMIT: Ticks = Ticks::MAX;

    fn price_limit_valid(price_limit: Ticks) -> bool {
        price_limit > Ticks::ZERO
    }

    fn get_quote_from_base_lots(
        size: BaseLots,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> Self::MatchingLots {
        (tick_size * price) * size
    }

    fn get_base_lots_from_quote(
        quote: Self::MatchingLots,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> BaseLots {
        quote / (tick_size * price)
    }

    fn get_opposite_quote(
        quote: Self::MatchingLots,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> <Self::Opposite as SideMarker>::MatchingLots {
        quote / (tick_size * price)
    }

    fn get_budget(num_lots: Self::Lots, base_lot_size: BaseLotsPerBaseUnit) -> Self::MatchingLots {
        num_lots * base_lot_size
    }

    fn get_lots_from_quote(
        quote: Self::MatchingLots,
        base_lot_size: BaseLotsPerBaseUnit,
    ) -> Self::Lots {
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

    fn atoms_per_lot(indexed_market: &IndexedMarket) -> Self::AtomsPerLot {
        indexed_market.quote_atoms_per_quote_lot()
    }

    fn consumed_for_side(market_delta: &mut MarketLotsDelta) -> &mut Self::DeltaLots {
        &mut market_delta.quote_lots_consumed
    }

    fn locked_for_side(market_delta: &mut MarketLotsDelta) -> &mut Self::DeltaLots {
        &mut market_delta.quote_lots_locked
    }

    fn update_maker_stores(
        base_store: &mut impl MakerStore,
        quote_store: &mut impl MakerStore,
        atoms: Self::Atoms,
        atoms_opposite: <Self::Opposite as SideMarker>::Atoms,
    ) {
        // For side bid, the maker is filling an ask.
        // Maker loses base and gains quote.
        // Subtraction is safe because backing assets are guaranteed.
        // Overflow on addition, i.e. maker overflowing to 0 balance is acceptable.
        base_store.reduce_locked(atoms_opposite.into());
        quote_store.add_free(atoms.into());
    }

    fn maker_update_for_side_ref<'a>(
        market_maker_delta: &'a MakerUpdate,
    ) -> &'a MakerUpdateSide<Self> {
        &market_maker_delta.bid
    }

    fn maker_update_for_side_mut<'a>(
        market_maker_delta: &'a mut MakerUpdate,
    ) -> &'a mut MakerUpdateSide<Self> {
        &mut market_maker_delta.bid
    }

    fn token_index_for_side(indexed_market: &IndexedMarket) -> TokenIndex {
        indexed_market.quote_token_index
    }
}

impl SideMarker for Ask {
    type Lots = BaseLots;
    type MatchingLots = BaseLots;
    type LotSize = BaseLotsPerBaseUnit;
    type Atoms = BaseAtoms;
    type AtomsPerLot = BaseAtomsPerBaseLot;
    type Opposite = Bid;

    const DEFAULT_PRICE_LIMIT: Ticks = Ticks::ZERO;

    fn price_limit_valid(_price_limit: Ticks) -> bool {
        true
    }

    fn get_quote_from_base_lots(
        size: BaseLots,
        _tick_size: QuoteLotsPerBaseUnitPerTick,
        _price: Ticks,
    ) -> Self::MatchingLots {
        size
    }

    fn get_base_lots_from_quote(
        quote: Self::MatchingLots,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> BaseLots {
        quote
    }

    fn get_opposite_quote(
        quote: Self::MatchingLots,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> <Self::Opposite as SideMarker>::MatchingLots {
        (tick_size * price) * quote
    }

    fn get_budget(num_lots: Self::Lots, _base_lot_size: BaseLotsPerBaseUnit) -> Self::MatchingLots {
        num_lots
    }

    fn get_lots_from_quote(
        quote: Self::MatchingLots,
        _base_lot_size: BaseLotsPerBaseUnit,
    ) -> Self::Lots {
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

    fn atoms_per_lot(indexed_market: &IndexedMarket) -> Self::AtomsPerLot {
        indexed_market.base_atoms_per_base_lot()
    }

    fn consumed_for_side(market_delta: &mut MarketLotsDelta) -> &mut Self::DeltaLots {
        &mut market_delta.base_lots_consumed
    }

    fn locked_for_side(market_delta: &mut MarketLotsDelta) -> &mut Self::DeltaLots {
        &mut market_delta.base_lots_locked
    }

    fn update_maker_stores(
        base_store: &mut impl MakerStore,
        quote_store: &mut impl MakerStore,
        atoms: Self::Atoms,
        atoms_opposite: <Self::Opposite as SideMarker>::Atoms,
    ) {
        // For side ask, the maker is filling a bid.
        // Maker gains base and loses quote.
        // Subtraction is safe because backing assets are guaranteed.
        // Overflow on addition, i.e. maker overflowing to 0 balance is acceptable.
        base_store.add_free(atoms.into());
        quote_store.reduce_locked(atoms_opposite.into());
    }

    fn maker_update_for_side_ref<'a>(
        market_maker_delta: &'a MakerUpdate,
    ) -> &'a MakerUpdateSide<Self> {
        &market_maker_delta.ask
    }

    fn maker_update_for_side_mut<'a>(
        market_maker_delta: &'a mut MakerUpdate,
    ) -> &'a mut MakerUpdateSide<Self> {
        &mut market_maker_delta.ask
    }

    fn token_index_for_side(indexed_market: &IndexedMarket) -> TokenIndex {
        indexed_market.base_token_index
    }
}
