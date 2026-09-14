use crate::quantities::{N1, P1, Unsided, Z0};

pub type UnsidedAtomsPerUnit<I> = Unsided<Z0, N1, P1, I>;
pub type UnsidedAtomsPerLot<I> = Unsided<N1, Z0, P1, I>;
pub type UnsidedLotsPerUnit<I> = Unsided<P1, N1, Z0, I>;
