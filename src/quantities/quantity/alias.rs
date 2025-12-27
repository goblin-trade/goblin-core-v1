use crate::quantities::{Dim, Quantity, SidedDim, N1, P1, Z0};

pub type BaseDim<L, U, A> = SidedDim<L, U, A>;
pub type QuoteDim<L, U, A> = SidedDim<L, U, A>;

pub type BaseLots = Quantity<Dim<BaseDim<P1, Z0, Z0>, QuoteDim<Z0, Z0, Z0>, Z0>>;
pub type BaseUnits = Quantity<Dim<BaseDim<Z0, P1, Z0>, QuoteDim<Z0, Z0, Z0>, Z0>>;
pub type BaseAtoms = Quantity<Dim<BaseDim<Z0, Z0, P1>, QuoteDim<Z0, Z0, Z0>, Z0>>;
pub type QuoteLots = Quantity<Dim<BaseDim<Z0, Z0, Z0>, QuoteDim<P1, Z0, Z0>, Z0>>;
pub type QuoteUnits = Quantity<Dim<BaseDim<Z0, Z0, Z0>, QuoteDim<Z0, P1, Z0>, Z0>>;
pub type QuoteAtoms = Quantity<Dim<BaseDim<Z0, Z0, Z0>, QuoteDim<Z0, Z0, P1>, Z0>>;
pub type Ticks = Quantity<Dim<BaseDim<Z0, Z0, Z0>, QuoteDim<Z0, Z0, Z0>, P1>>;

// Binary ratios
pub type BaseLotsPerBaseUnit = Quantity<Dim<BaseDim<P1, N1, Z0>, QuoteDim<Z0, Z0, Z0>, Z0>>;
pub type QuoteLotsPerQuoteUnit = Quantity<Dim<BaseDim<Z0, Z0, Z0>, QuoteDim<P1, N1, Z0>, Z0>>;
pub type QuoteLotsPerBaseUnit = Quantity<Dim<BaseDim<Z0, N1, Z0>, QuoteDim<P1, Z0, Z0>, Z0>>;

pub type BaseAtomsPerBaseUnit = Quantity<Dim<BaseDim<Z0, N1, P1>, QuoteDim<Z0, Z0, Z0>, Z0>>;
pub type QuoteAtomsPerQuoteUnit = Quantity<Dim<BaseDim<Z0, Z0, Z0>, QuoteDim<Z0, N1, P1>, Z0>>;

pub type BaseAtomsPerBaseLot = Quantity<Dim<BaseDim<N1, Z0, P1>, QuoteDim<Z0, Z0, Z0>, Z0>>;
pub type QuoteAtomsPerQuoteLot = Quantity<Dim<BaseDim<Z0, Z0, Z0>, QuoteDim<N1, Z0, P1>, Z0>>;

// Tertiary
pub type QuoteLotsPerBaseUnitPerTick = Quantity<Dim<BaseDim<Z0, N1, Z0>, QuoteDim<P1, Z0, Z0>, N1>>;
pub type QuoteLotsPerBaseLotPerTick = Quantity<Dim<BaseDim<N1, Z0, Z0>, QuoteDim<P1, Z0, Z0>, N1>>;
pub type AdjustedQuoteLots = Quantity<Dim<BaseDim<P1, N1, Z0>, QuoteDim<P1, Z0, Z0>, Z0>>;
