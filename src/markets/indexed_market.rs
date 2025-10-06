use crate::{
    quantities::{QuantityOps, QuoteLotsPerBaseUnitPerTick},
    tokens::DynamicIndex,
    types::{Base, LegMarker, Pair, Quote},
};

// Max number of custom markets
pub const MAX_CUSTOM_MARKETS: usize = 7;

pub type TokenIndexPair = Pair<DynamicIndex, DynamicIndex>;
pub type LotSizePair = Pair<<Base as LegMarker>::LotsPerUnit, <Quote as LegMarker>::LotsPerUnit>;

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct IndexedMarket {
    pub token_index_pair: TokenIndexPair,

    pub lot_size_pair: LotSizePair,

    /// Tick size
    pub tick_size: QuoteLotsPerBaseUnitPerTick,
}

impl IndexedMarket {
    pub fn base_lot_size(&self) -> <Base as LegMarker>::LotsPerUnit {
        self.lot_size_pair.base
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
    pub fn is_valid(&self) -> bool {
        // TODO use specialized IndexedMarket types for ERC20-ETH and ERC20-ERC20
        self.token_index_pair.base != self.token_index_pair.quote
            && Base::lots_per_unit_valid(self.lot_size_pair.base)
            && Quote::lots_per_unit_valid(self.lot_size_pair.quote)
            && self.tick_size % self.base_lot_size() == QuoteLotsPerBaseUnitPerTick::ZERO
    }
}
