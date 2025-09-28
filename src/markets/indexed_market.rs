use crate::{
    quantities::{QuantityOps, QuoteLotsPerBaseUnitPerTick},
    tokens::TokenIndex,
    types::{Base, LegMarker, Quote},
};

// Max number of custom markets
pub const MAX_CUSTOM_MARKETS: usize = 7;

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct MarketLeg<L: LegMarker> {
    /// The token index. It will be mapped to token address.
    pub token_index: TokenIndex,

    /// Lots per unit
    pub lot_size: L::LotsPerUnit,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct IndexedMarket {
    /// Marker parameters for the base leg
    pub base: MarketLeg<Base>,

    /// Marker parameters for the quote leg
    pub quote: MarketLeg<Quote>,

    /// Tick size
    pub tick_size: QuoteLotsPerBaseUnitPerTick,
}

impl IndexedMarket {
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
        self.base.token_index != self.quote.token_index
            && Base::lots_per_unit_valid(self.base.lot_size)
            && Quote::lots_per_unit_valid(self.quote.lot_size)
            && self.tick_size % self.base.lot_size == QuoteLotsPerBaseUnitPerTick::ZERO
    }
}
