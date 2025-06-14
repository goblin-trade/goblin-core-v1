use crate::{
    quantities::{BaseLots, BaseLotsPerBaseUnit, QuoteLots, QuoteLotsPerBaseUnitPerTick},
    types::Address,
};

/// Slot key of a market
///
/// This struct groups parameters that must satisfy two core arithmetic invariants:
///
/// 1. **Minimum decimals check**
///    We universally fix “atoms per lot” to 10⁶ for every token.
///    To ensure the on‑chain mint’s decimals can cleanly subdivide into 10⁶‑atom lots,
///    we require:
///    ```text
///    10^decimals % (10^decimals / 10^6) == 0
///    ```
///    Hence any token mint must declare at least 6 decimals, so that
///    `base_atoms_per_base_unit / atoms_per_lot` is an integer.
///
/// 2. **Tick‑vs‑lot invariant**
///    Let:
///      - `B` = base lots per base unit
///      - `T` = quote lots per tick (per base unit)
///    A trade of **1 base lot** at **1 tick** price must yield an integer number of quote lots:
///    ```text
///    N = T (quote lots/tick)
///        × 1 (tick/base unit)
///        ÷ B (base lots/base unit)
///      = T / B ∈ ℤ
///    ```
///    Therefore we assert:
///    ```rust
///    assert!(T % B == 0, "Tick size must be a multiple of base lots per unit");
///    ```
#[repr(C)]
pub struct MarketStateKey {
    /// The base token
    pub base_token_index: u8,

    /// The quote token
    pub quote_token_index: u8,

    /// Base atoms per lot
    // pub base_lot_size: BaseAtomsPerBaseLot,

    /// Quote atoms per lot
    // pub quote_lot_size: QuoteAtomsPerQuoteLot,

    /// Ticks increment by these many quote lots
    /// Eg. say quote lot size is 0.01 USDC, i.e. 100 quote lots per unit
    /// And the tick size is 0.1 USDC per base unit. i.e 10 quote lots per base lot
    ///
    /// # Orderbook invariant
    ///
    /// * tick_size % base_lots_per_base_unit = 0
    /// * In the above example 10 % 10 = 0 which is valid
    pub tick_size: QuoteLotsPerBaseUnitPerTick,
}

impl MarketStateKey {
    // fn is_valid(&self) -> bool {
    //     // 2. base_atoms_per_base_unit % num_base_lots_per_base_unit == 0
    //     // 10^6 % (10^6 / x) == 0
    //     // This can be valid only if the denominator (10^6 / x) is an integer
    //     // That is 10^6 % x == 0
    //     let base_atoms_per_base_unit = 1_000_000;
    //     let num_base_lots_per_base_unit = 1_000_000 / self.base_lot_size.0;
    //     if base_atoms_per_base_unit % num_base_lots_per_base_unit != 0 {
    //         return false;
    //     }

    //     // Every token has the same unit size- 10^6 atoms per unit
    //     // We can have variable lot sizes, i.e. different atoms per 'lot'
    //     // For example we could have 10 ETH atoms per lot or 100 atoms per lot
    //     //
    //     // Legal values are 2, 5, 10, 25 etc. Multiples of 5 and 2.
    //     // In practice it will be 10^n where n <= 6
    //     //
    //     if 1_000_000 % self.base_lot_size.0 != 0 {
    //         return false;
    //     }

    //     // Similarly for quote atoms

    //     // 3. tick size in quote lots % num_base_lots_per_base_unit == 0
    //     // Equivalent to tick size in quote atoms % num_base_lots_per_base_unit == 0

    //     true
    // }

    // fn try_new(
    //     base_token: Address,
    //     quote_token: Address,
    //     base_lots_per_base_unit: BaseLotsPerBaseUnit,
    //     tick_size: QuoteLotsPerBaseUnitPerTick,
    // ) -> Result<Self, ()> {
    //     if base_lots_per_base_unit % tick_size {
    //         Ok(MarketStateKey {
    //             base_token,
    //             quote_token,
    //             base_lots_per_base_unit,
    //             tick_size,
    //         })
    //     } else {
    //         Err(())
    //     }
    // }
}
