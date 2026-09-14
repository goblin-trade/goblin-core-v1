use crate::{
    axis::leg::{Base, Quote},
    quantities::{Dim, N1, P1, Quantity, SidedDim, Z0},
};

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
