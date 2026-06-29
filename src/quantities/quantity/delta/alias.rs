use crate::quantities::{BaseDim, Dim, Quantity, QuoteDim, Unsided, N1, P1, Z0};

pub type BaseDeltaAtoms = Quantity<Dim<BaseDim<Z0, Z0, P1>, QuoteDim<Z0, Z0, Z0>, Z0>, i64>;
pub type QuoteDeltaAtoms = Quantity<Dim<BaseDim<Z0, Z0, Z0>, QuoteDim<Z0, Z0, P1>, Z0>, i64>;

pub type BaseDeltaLots = Quantity<Dim<BaseDim<P1, Z0, Z0>, QuoteDim<Z0, Z0, Z0>, Z0>, i64>;
pub type QuoteDeltaLots = Quantity<Dim<BaseDim<Z0, Z0, Z0>, QuoteDim<P1, Z0, Z0>, Z0>, i64>;

////// Unsided
pub type UnsidedDeltaAtoms = Unsided<Z0, Z0, P1, i64>;
pub type UnsidedDeltaLots = Unsided<P1, Z0, Z0, i64>;

pub type UnsidedDeltaAtomsPerUnit = Unsided<Z0, N1, P1, i64>;
pub type UnsidedDeltaAtomsPerLot = Unsided<N1, Z0, P1, i64>;
pub type UnsidedDeltaLotsPerUnit = Unsided<P1, N1, Z0, i64>;
