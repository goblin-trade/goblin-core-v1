use crate::{
    axis::leg::{Base, Quote},
    quantities::{Dim, N1, P1, Quantity, SidedDim, Z0},
};

// Binary ratios
pub type BaseLotsPerBaseUnit<I> =
    Quantity<Dim<SidedDim<Base, P1, N1, Z0>, SidedDim<Quote, Z0, Z0, Z0>, Z0>, I>;
pub type QuoteLotsPerQuoteUnit<I> =
    Quantity<Dim<SidedDim<Base, Z0, Z0, Z0>, SidedDim<Quote, P1, N1, Z0>, Z0>, I>;
pub type QuoteLotsPerBaseUnit<I> =
    Quantity<Dim<SidedDim<Base, Z0, N1, Z0>, SidedDim<Quote, P1, Z0, Z0>, Z0>, I>;

pub type BaseAtomsPerBaseUnit<I> =
    Quantity<Dim<SidedDim<Base, Z0, N1, P1>, SidedDim<Quote, Z0, Z0, Z0>, Z0>, I>;
pub type QuoteAtomsPerQuoteUnit<I> =
    Quantity<Dim<SidedDim<Base, Z0, Z0, Z0>, SidedDim<Quote, Z0, N1, P1>, Z0>, I>;

pub type BaseAtomsPerBaseLot<I> =
    Quantity<Dim<SidedDim<Base, N1, Z0, P1>, SidedDim<Quote, Z0, Z0, Z0>, Z0>, I>;
pub type QuoteAtomsPerQuoteLot<I> =
    Quantity<Dim<SidedDim<Base, Z0, Z0, Z0>, SidedDim<Quote, N1, Z0, P1>, Z0>, I>;
