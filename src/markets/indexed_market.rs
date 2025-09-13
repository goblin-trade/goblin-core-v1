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
pub struct IndexedMarketV2 {
    /// Marker parameters for the base leg
    pub base: MarketLeg<Base>,

    /// Marker parameters for the quote leg
    pub quote: MarketLeg<Quote>,

    /// Tick size
    pub tick_size: QuoteLotsPerBaseUnitPerTick,
}

impl IndexedMarketV2 {
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
        self.base.token_index != self.quote.token_index
            && Base::lots_per_unit_valid(self.base.lot_size)
            && Quote::lots_per_unit_valid(self.quote.lot_size)
            && self.tick_size % self.base.lot_size == QuoteLotsPerBaseUnitPerTick::ZERO
    }
}

#[derive(Default)]
pub struct LegLotsDelta<L: LegMarker> {
    pub consumed: L::LotsDelta,
    pub locked: L::LotsDelta,
}

impl<L: LegMarker> LegLotsDelta<L> {
    pub fn to_atoms_delta(&self, atoms_per_lot: L::AtomsPerLot) -> LegAtomsDelta<L> {
        LegAtomsDelta {
            consumed: self.consumed * atoms_per_lot,
            locked: self.locked * atoms_per_lot,
        }
    }
}

/// Consumed and locked deltas of msg.sender for a market
#[derive(Default)]
pub struct MarketLotsDelta {
    pub base: LegLotsDelta<Base>,
    pub quote: LegLotsDelta<Quote>,
}

pub struct LegAtomsDelta<L: LegMarker> {
    pub consumed: L::AtomsDelta,
    pub locked: L::AtomsDelta,
}

pub struct MarketAtomsDelta {
    pub base: LegAtomsDelta<Base>,
    pub quote: LegAtomsDelta<Quote>,
}

// pub struct MarketAtomsDelta {
//     pub base_atoms_consumed: AtomsDelta,
//     pub quote_atoms_consumed: AtomsDelta,
//     pub base_atoms_locked: AtomsDelta,
//     pub quote_atoms_locked: AtomsDelta,
// }

// /// Parameters representing a Goblin market.
// #[repr(C, packed)]
// #[derive(Clone, Copy)]
// pub struct IndexedMarket {
//     /// The base token index. It will be mapped to token address
//     pub base_token_index: TokenIndex,

//     /// The quote token index. It will be mapped to token address
//     pub quote_token_index: TokenIndex,

//     /// Base lots per unit
//     pub base_lot_size: BaseLotsPerBaseUnit,

//     /// Quote lots per unit
//     pub quote_lot_size: QuoteLotsPerQuoteUnit,

//     /// Tick size
//     pub tick_size: QuoteLotsPerBaseUnitPerTick,
// }

// impl IndexedMarket {
//     /// Initialize an IndexedMarket, bypassing legality checks.
//     /// This function is used to define hardcoded markets
//     pub(crate) const fn new_unchecked(
//         base_token_index: TokenIndex,
//         quote_token_index: TokenIndex,
//         base_lot_size: BaseLotsPerBaseUnit,
//         quote_lot_size: QuoteLotsPerQuoteUnit,
//         tick_size: QuoteLotsPerBaseUnitPerTick,
//     ) -> Self {
//         IndexedMarket {
//             base_token_index,
//             quote_token_index,
//             base_lot_size,
//             quote_lot_size,
//             tick_size,
//         }
//     }

//     /// Whether market params are valid
//     ///
//     /// # Tests
//     ///
//     /// 1. Base token != Quote token
//     ///
//     /// 2. **10^6 % Lot size == 0**
//     ///
//     ///   Since we normalize tokens to 6 decimal places, 1 unit holds 10^6 atoms. Therefore lot
//     ///   size should divide 10^6.
//     ///
//     /// 3. T % B == 0, the tick-vs-lot invariant
//     ///    Let:
//     ///      - `B` = base lots per base unit
//     ///      - `T` = quote lots per tick (per base unit)
//     ///    A trade of **1 base lot** at **1 tick** price must yield an integer number of quote lots:
//     ///    ```text
//     ///    N = T (quote lots/tick)
//     ///        × 1 (tick/base unit)
//     ///        ÷ B (base lots/base unit)
//     ///      = T / B ∈ ℤ
//     ///
//     ///   or T % B == 0
//     ///    ```
//     ///
//     pub(crate) fn is_valid(&self) -> bool {
//         let base_lot_size = self.base_lot_size;
//         let quote_lot_size = self.quote_lot_size;

//         self.base_token_index != self.quote_token_index
//             && BASE_ATOMS_PER_BASE_UNIT % base_lot_size == BaseAtomsPerBaseLot::ZERO
//             && QUOTE_ATOMS_PER_QUOTE_UNIT % quote_lot_size == QuoteAtomsPerQuoteLot::ZERO
//             && self.tick_size % self.base_lot_size == QuoteLotsPerBaseLotPerTick::ZERO
//     }

//     // Atoms per lot are guaranteed to be whole numbers because of the validation check above
//     pub fn base_atoms_per_base_lot(&self) -> BaseAtomsPerBaseLot {
//         BASE_ATOMS_PER_BASE_UNIT / self.base_lot_size
//     }

//     pub fn quote_atoms_per_quote_lot(&self) -> QuoteAtomsPerQuoteLot {
//         QUOTE_ATOMS_PER_QUOTE_UNIT / self.quote_lot_size
//     }

//     /// Convert market lot delta to atom delta
//     pub fn get_atoms_delta(&self, market_delta: &MarketLotsDelta) -> MarketAtomsDelta {
//         let base_atoms_per_base_lot = self.base_atoms_per_base_lot();
//         let quote_atoms_per_quote_lot = self.quote_atoms_per_quote_lot();

//         let base_atoms_consumed = base_atoms_per_base_lot * market_delta.base_lots_consumed;
//         let quote_atoms_consumed = quote_atoms_per_quote_lot * market_delta.quote_lots_consumed;

//         let base_atoms_locked = base_atoms_per_base_lot * market_delta.base_lots_locked;
//         let quote_atoms_locked = quote_atoms_per_quote_lot * market_delta.quote_lots_locked;

//         MarketAtomsDelta {
//             base_atoms_consumed,
//             quote_atoms_consumed,
//             base_atoms_locked,
//             quote_atoms_locked,
//         }
//     }
// }

// pub struct MarketAtomsDelta {
//     pub base_atoms_consumed: AtomsDelta,
//     pub quote_atoms_consumed: AtomsDelta,
//     pub base_atoms_locked: AtomsDelta,
//     pub quote_atoms_locked: AtomsDelta,
// }
