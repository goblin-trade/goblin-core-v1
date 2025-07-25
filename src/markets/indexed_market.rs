use crate::{
    goblin_error::GoblinError,
    markets::HARDCODED_MARKETS,
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit},
    require,
    tokens::TokenIndex,
    types::Address,
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

    /// Obtain an IndexedMarket from market index number.
    /// We use the index number to lookup in custom and hardcoded market lists.
    /// Custom markets are tested for validity.
    ///
    /// # Arguments
    ///
    /// * `index`- Market index
    /// * `dangerous_custom_market_list` - Market list read from args
    pub fn from_index(
        index: usize,
        dangerous_custom_market_list: &[IndexedMarket],
    ) -> Result<Self, GoblinError> {
        if index < dangerous_custom_market_list.len() {
            let dangerous_custom_market = dangerous_custom_market_list[index];
            require!(
                dangerous_custom_market.is_valid(),
                GoblinError::InvalidMarket
            );

            Ok(dangerous_custom_market)
        } else if index > 127 && index < (127 + HARDCODED_MARKETS.len()) {
            Ok(HARDCODED_MARKETS[index - 127])
        } else {
            Err(GoblinError::NoMarketAtIndex)
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
    fn is_valid(&self) -> bool {
        let base_lot_size = self.base_lot_size;
        let quote_lot_size = self.quote_lot_size;

        self.base_token_index != self.quote_token_index
            && base_lot_size.valid()
            && quote_lot_size.valid()
            && self.tick_size % self.base_lot_size == 0
    }
}

#[derive(Clone, Copy)]
pub struct Market {
    pub base_token: Address,
    pub quote_token: Address,
    pub base_lot_size: BaseLotsPerBaseUnit,
    pub quote_lot_size: QuoteLotsPerQuoteUnit,
    pub tick_size: QuoteLotsPerBaseUnitPerTick,
}
