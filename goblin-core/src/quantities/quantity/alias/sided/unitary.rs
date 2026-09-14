use crate::{
    axis::leg::{Base, Quote},
    quantities::{Dim, P1, Quantity, SidedDim, Z0},
};

pub type BaseLots<I> =
    Quantity<Dim<SidedDim<Base, P1, Z0, Z0>, SidedDim<Quote, Z0, Z0, Z0>, Z0>, I>;
pub type BaseUnits<I> =
    Quantity<Dim<SidedDim<Base, Z0, P1, Z0>, SidedDim<Quote, Z0, Z0, Z0>, Z0>, I>;
pub type BaseAtoms<I> =
    Quantity<Dim<SidedDim<Base, Z0, Z0, P1>, SidedDim<Quote, Z0, Z0, Z0>, Z0>, I>;

pub type QuoteLots<I> =
    Quantity<Dim<SidedDim<Base, Z0, Z0, Z0>, SidedDim<Quote, P1, Z0, Z0>, Z0>, I>;
pub type QuoteUnits<I> =
    Quantity<Dim<SidedDim<Base, Z0, Z0, Z0>, SidedDim<Quote, Z0, P1, Z0>, Z0>, I>;
pub type QuoteAtoms<I> =
    Quantity<Dim<SidedDim<Base, Z0, Z0, Z0>, SidedDim<Quote, Z0, Z0, P1>, Z0>, I>;

pub type Ticks = Quantity<Dim<SidedDim<Base, Z0, Z0, Z0>, SidedDim<Quote, Z0, Z0, Z0>, P1>, u64>;
