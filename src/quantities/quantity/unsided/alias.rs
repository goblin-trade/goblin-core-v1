///! Unsided atoms are used in the global delta where there is no notion of side
use crate::quantities::{Quantity, SidedDim, P1, Z0};

pub type Unsided<L, U, A> = Quantity<SidedDim<L, U, A>>;
pub type UnsidedAtoms = Unsided<Z0, Z0, P1>;
