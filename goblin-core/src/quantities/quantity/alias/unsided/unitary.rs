use crate::quantities::{P1, Unsided, Z0};

pub type UnsidedAtoms<I> = Unsided<Z0, Z0, P1, I>;
pub type UnsidedLots<I> = Unsided<P1, Z0, Z0, I>;
