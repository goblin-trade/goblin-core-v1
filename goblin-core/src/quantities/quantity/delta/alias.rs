use crate::{
    axis::leg::{Base, Quote},
    quantities::{Dim, N1, P1, Quantity, SidedDim, Unsided, Z0},
};

pub type BaseDeltaAtoms =
    Quantity<Dim<SidedDim<Base, Z0, Z0, P1>, SidedDim<Quote, Z0, Z0, Z0>, Z0>, i64>;
pub type QuoteDeltaAtoms =
    Quantity<Dim<SidedDim<Base, Z0, Z0, Z0>, SidedDim<Quote, Z0, Z0, P1>, Z0>, i64>;

pub type BaseDeltaLots =
    Quantity<Dim<SidedDim<Base, P1, Z0, Z0>, SidedDim<Quote, Z0, Z0, Z0>, Z0>, i64>;
pub type QuoteDeltaLots =
    Quantity<Dim<SidedDim<Base, Z0, Z0, Z0>, SidedDim<Quote, P1, Z0, Z0>, Z0>, i64>;
