use crate::{
    goblin_error::GoblinError,
    markets::IndexedMarket,
    quantities::{
        AdjustedQuoteLots, BaseAtoms, BaseAtomsPerBaseLot, BaseLots, BaseLotsDelta,
        BaseLotsPerBaseUnit, MarketLotsDelta, QuoteAtoms, QuoteAtomsPerQuoteLot, QuoteLots,
        QuoteLotsDelta, QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit, Ticks,
    },
    state::{ERC20Store, ERC20StoreKey, EthStore, EthStoreKey, MarketState, SlotState},
    tokens::{ERC20TokenPair, ValidatedTokenPair},
    types::Address,
};

pub struct Bid;
pub struct Ask;

pub trait SideMarker {
    // The input lots for a take order of this side
    type Lots: Copy
        + PartialOrd
        + Default
        + From<u64>
        + core::ops::Mul<Self::AtomsPerLot, Output = Self::Atoms>;

    type DeltaLots: Copy
        + core::ops::Add<
            Self::Lots,
            Output = Result<Self::DeltaLots, crate::goblin_error::GoblinError>,
        > + core::ops::Sub<
            Self::Lots,
            Output = Result<Self::DeltaLots, crate::goblin_error::GoblinError>,
        >;

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

    // Update token stores for the maker where 'S' is the taker side.
    // - If side is Bid, the maker is executing an Ask.
    // - If side is Ask, the maker is executing a Bid.
    fn update_maker_stores(
        token_pair: &ValidatedTokenPair,
        maker: &Address,
        atoms: Self::Atoms,
        atoms_opposite: <Self::Opposite as SideMarker>::Atoms,
    ) -> Result<(), GoblinError>;
}

impl SideMarker for Bid {
    type Lots = QuoteLots;
    type DeltaLots = QuoteLotsDelta;
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
        token_pair: &ValidatedTokenPair,
        maker: &Address,
        atoms: Self::Atoms,
        atoms_opposite: <Self::Opposite as SideMarker>::Atoms,
    ) -> Result<(), GoblinError> {
        match token_pair {
            ValidatedTokenPair::ERC20ERC20(ERC20TokenPair {
                base_token,
                quote_token,
            }) => {
                // Side- bid
                // The maker is filling an ask
                // Maker gains quote and loses base
                // Subtraction is safe because backing assets are guaranteed
                let base_token_key = ERC20StoreKey::new(maker, base_token.address());
                let mut base_token_store = ERC20Store::load(&base_token_key);
                base_token_store.as_mut().atoms_locked -= atoms_opposite.into();
                base_token_store.as_mut().store(&base_token_key);

                // Overflow on adding is acceptable. The taker's balance will be wiped.
                let quote_token_key = ERC20StoreKey::new(maker, quote_token.address());
                let mut quote_token_store = ERC20Store::load(&quote_token_key);
                quote_token_store.as_mut().atoms_free += atoms.into();
                quote_token_store.as_mut().store(&quote_token_key);
            }
            ValidatedTokenPair::ETHERC20(quote_token) => {
                let base_token_key = EthStoreKey::new(maker);
                let mut base_token_store = EthStore::load(&base_token_key);
                base_token_store.as_mut().atoms_locked -= atoms_opposite.into();
                base_token_store.as_mut().store(&base_token_key);

                let quote_token_key = ERC20StoreKey::new(maker, quote_token.address());
                let mut quote_token_store = ERC20Store::load(&quote_token_key);
                quote_token_store.as_mut().atoms_free += atoms.into();
                quote_token_store.as_mut().store(&quote_token_key);
            }
            ValidatedTokenPair::ERC20ETH(base_token) => {
                let base_token_key = ERC20StoreKey::new(maker, base_token.address());
                let mut base_token_store = ERC20Store::load(&base_token_key);
                base_token_store.as_mut().atoms_locked -= atoms_opposite.into();
                base_token_store.as_mut().store(&base_token_key);

                let quote_token_key = EthStoreKey::new(maker);
                let mut quote_token_store = EthStore::load(&quote_token_key);
                quote_token_store.as_mut().atoms_free += atoms.into();
                quote_token_store.as_mut().store(&quote_token_key);
            }
        }
        Ok(())
    }
}

impl SideMarker for Ask {
    type Lots = BaseLots;
    type DeltaLots = BaseLotsDelta;
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
        token_pair: &ValidatedTokenPair,
        maker: &Address,
        atoms: Self::Atoms,
        atoms_opposite: <Self::Opposite as SideMarker>::Atoms,
    ) -> Result<(), GoblinError> {
        match token_pair {
            ValidatedTokenPair::ERC20ERC20(ERC20TokenPair {
                base_token,
                quote_token,
            }) => {
                // Side- ask
                // The maker is filling a bid
                // Maker gains base and loses quote
                let base_token_key = ERC20StoreKey::new(maker, base_token.address());
                let mut base_token_store = ERC20Store::load(&base_token_key);
                base_token_store.as_mut().atoms_free += atoms.into();
                base_token_store.as_mut().store(&base_token_key);

                let quote_token_key = ERC20StoreKey::new(maker, quote_token.address());
                let mut quote_token_store = ERC20Store::load(&quote_token_key);
                quote_token_store.as_mut().atoms_locked -= atoms_opposite.into();
                quote_token_store.as_mut().store(&quote_token_key);
            }
            ValidatedTokenPair::ETHERC20(quote_token) => {
                let base_token_key = EthStoreKey::new(maker);
                let mut base_token_store = EthStore::load(&base_token_key);
                base_token_store.as_mut().atoms_free += atoms.into();
                base_token_store.as_mut().store(&base_token_key);

                let quote_token_key = ERC20StoreKey::new(maker, quote_token.address());
                let mut quote_token_store = ERC20Store::load(&quote_token_key);
                quote_token_store.as_mut().atoms_locked -= atoms_opposite.into();
                quote_token_store.as_mut().store(&quote_token_key);
            }
            ValidatedTokenPair::ERC20ETH(base_token) => {
                let base_token_key = ERC20StoreKey::new(maker, base_token.address());
                let mut base_token_store = ERC20Store::load(&base_token_key);
                base_token_store.as_mut().atoms_free += atoms.into();
                base_token_store.as_mut().store(&base_token_key);

                let quote_token_key = EthStoreKey::new(maker);
                let mut quote_token_store = EthStore::load(&quote_token_key);
                quote_token_store.as_mut().atoms_locked -= atoms_opposite.into();
                quote_token_store.as_mut().store(&quote_token_key);
            }
        }
        Ok(())
    }
}
