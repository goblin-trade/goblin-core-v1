use crate::{
    quantities::{
        AtomsDelta, BaseAtomsPerBaseLot, BaseLotsPerBaseUnit, MarketLotsDelta,
        QuoteAtomsPerQuoteLot, QuoteLotsPerBaseLotsPerTick, QuoteLotsPerBaseUnitPerTick,
        QuoteLotsPerQuoteUnit, BASE_ATOMS_PER_BASE_UNIT, QUOTE_ATOMS_PER_QUOTE_UNIT,
    },
    tokens::TokenIndex,
};

// Max number of custom markets
pub const MAX_CUSTOM_MARKETS: usize = 7;

/// Parameters representing a Goblin market.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct IndexedMarket {
    /// The base token index. It will be mapped to token address
    pub base_token_index: TokenIndex,

    /// The quote token index. It will be mapped to token address
    pub quote_token_index: TokenIndex,

    /// Base lots per unit
    pub base_lot_size: BaseLotsPerBaseUnit,

    /// Quote lots per unit
    pub quote_lot_size: QuoteLotsPerQuoteUnit,

    /// Tick size
    pub tick_size: QuoteLotsPerBaseUnitPerTick,
}

impl IndexedMarket {
    /// Initialize an IndexedMarket, bypassing legality checks.
    /// This function is used to define hardcoded markets
    pub(crate) const fn new_unchecked(
        base_token_index: TokenIndex,
        quote_token_index: TokenIndex,
        base_lot_size: BaseLotsPerBaseUnit,
        quote_lot_size: QuoteLotsPerQuoteUnit,
        tick_size: QuoteLotsPerBaseUnitPerTick,
    ) -> Self {
        IndexedMarket {
            base_token_index,
            quote_token_index,
            base_lot_size,
            quote_lot_size,
            tick_size,
        }
    }

    /// Whether market params are valid
    ///
    /// # Tests
    ///
    /// 1. Base token != Quote token
    ///
    /// 2. **10^6 % Lot size == 0**
    ///
    ///   Since we normalize tokens to 6 decimal places, 1 unit holds 10^6 atoms. Therefore lot
    ///   size should divide 10^6.
    ///
    /// 3. T % B == 0, the tick-vs-lot invariant
    ///    Let:
    ///      - `B` = base lots per base unit
    ///      - `T` = quote lots per tick (per base unit)
    ///    A trade of **1 base lot** at **1 tick** price must yield an integer number of quote lots:
    ///    ```text
    ///    N = T (quote lots/tick)
    ///        × 1 (tick/base unit)
    ///        ÷ B (base lots/base unit)
    ///      = T / B ∈ ℤ
    ///
    ///   or T % B == 0
    ///    ```
    ///
    pub(crate) fn is_valid(&self) -> bool {
        let base_lot_size = self.base_lot_size;
        let quote_lot_size = self.quote_lot_size;

        self.base_token_index != self.quote_token_index
            && BASE_ATOMS_PER_BASE_UNIT % base_lot_size == BaseAtomsPerBaseLot::ZERO
            && QUOTE_ATOMS_PER_QUOTE_UNIT % quote_lot_size == QuoteAtomsPerQuoteLot::ZERO
            && self.tick_size % self.base_lot_size == QuoteLotsPerBaseLotsPerTick::ZERO
    }

    /// Convert market lot delta to atom delta
    pub fn get_atoms_delta(&self, market_delta: &MarketLotsDelta) -> MarketAtomsDelta {
        // Atoms per lot are guaranteed to be whole numbers because of the validation check above
        let base_atoms_per_base_lot = BASE_ATOMS_PER_BASE_UNIT / self.base_lot_size;
        let quote_atoms_per_quote_lot = QUOTE_ATOMS_PER_QUOTE_UNIT / self.quote_lot_size;

        let base_atoms_delta = base_atoms_per_base_lot * market_delta.base_lots_delta;
        let quote_atoms_delta = quote_atoms_per_quote_lot * market_delta.quote_lots_delta;

        MarketAtomsDelta {
            base_atoms_delta,
            quote_atoms_delta,
        }
    }
}

pub struct MarketAtomsDelta {
    pub base_atoms_delta: AtomsDelta,
    pub quote_atoms_delta: AtomsDelta,
}
