use crate::{
    axis::leg::{Base, Quote},
    quantities::{Dim, N1, P1, Quantity, SidedDim, Z0},
};

// Tertiary
pub type QuoteLotsPerBaseUnitPerTick<I> =
    Quantity<Dim<SidedDim<Base, Z0, N1, Z0>, SidedDim<Quote, P1, Z0, Z0>, N1>, I>;
pub type QuoteLotsPerBaseLotPerTick =
    Quantity<Dim<SidedDim<Base, N1, Z0, Z0>, SidedDim<Quote, P1, Z0, Z0>, N1>, u64>;
pub type AdjustedQuoteLots =
    Quantity<Dim<SidedDim<Base, P1, N1, Z0>, SidedDim<Quote, P1, Z0, Z0>, Z0>, u64>;
