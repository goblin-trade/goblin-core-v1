use crate::{
    axis::leg::{Base, Quote},
    quantities::{Dim, N1, P1, Quantity, SidedDim, Z0},
};

pub type BaseLots<I> =
    Quantity<Dim<SidedDim<Base, P1, Z0, Z0>, SidedDim<Quote, Z0, Z0, Z0>, Z0>, I>;
pub type BaseUnits =
    Quantity<Dim<SidedDim<Base, Z0, P1, Z0>, SidedDim<Quote, Z0, Z0, Z0>, Z0>, u64>;
pub type BaseAtoms<I> =
    Quantity<Dim<SidedDim<Base, Z0, Z0, P1>, SidedDim<Quote, Z0, Z0, Z0>, Z0>, I>;

pub type QuoteLots<I> =
    Quantity<Dim<SidedDim<Base, Z0, Z0, Z0>, SidedDim<Quote, P1, Z0, Z0>, Z0>, I>;
pub type QuoteUnits =
    Quantity<Dim<SidedDim<Base, Z0, Z0, Z0>, SidedDim<Quote, Z0, P1, Z0>, Z0>, u64>;
pub type QuoteAtoms<I> =
    Quantity<Dim<SidedDim<Base, Z0, Z0, Z0>, SidedDim<Quote, Z0, Z0, P1>, Z0>, I>;
pub type Ticks = Quantity<Dim<SidedDim<Base, Z0, Z0, Z0>, SidedDim<Quote, Z0, Z0, Z0>, P1>, u64>;

// Binary ratios
pub type BaseLotsPerBaseUnit =
    Quantity<Dim<SidedDim<Base, P1, N1, Z0>, SidedDim<Quote, Z0, Z0, Z0>, Z0>, u64>;
pub type QuoteLotsPerQuoteUnit =
    Quantity<Dim<SidedDim<Base, Z0, Z0, Z0>, SidedDim<Quote, P1, N1, Z0>, Z0>, u64>;
pub type QuoteLotsPerBaseUnit =
    Quantity<Dim<SidedDim<Base, Z0, N1, Z0>, SidedDim<Quote, P1, Z0, Z0>, Z0>, u64>;

pub type BaseAtomsPerBaseUnit =
    Quantity<Dim<SidedDim<Base, Z0, N1, P1>, SidedDim<Quote, Z0, Z0, Z0>, Z0>, u64>;
pub type QuoteAtomsPerQuoteUnit =
    Quantity<Dim<SidedDim<Base, Z0, Z0, Z0>, SidedDim<Quote, Z0, N1, P1>, Z0>, u64>;

pub type BaseAtomsPerBaseLot =
    Quantity<Dim<SidedDim<Base, N1, Z0, P1>, SidedDim<Quote, Z0, Z0, Z0>, Z0>, u64>;
pub type QuoteAtomsPerQuoteLot =
    Quantity<Dim<SidedDim<Base, Z0, Z0, Z0>, SidedDim<Quote, N1, Z0, P1>, Z0>, u64>;

// Tertiary
pub type QuoteLotsPerBaseUnitPerTick =
    Quantity<Dim<SidedDim<Base, Z0, N1, Z0>, SidedDim<Quote, P1, Z0, Z0>, N1>, u64>;
pub type QuoteLotsPerBaseLotPerTick =
    Quantity<Dim<SidedDim<Base, N1, Z0, Z0>, SidedDim<Quote, P1, Z0, Z0>, N1>, u64>;
pub type AdjustedQuoteLots =
    Quantity<Dim<SidedDim<Base, P1, N1, Z0>, SidedDim<Quote, P1, Z0, Z0>, Z0>, u64>;
