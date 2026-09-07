use crate::quantities::{QuantityOps, TryIntoUnsidedDelta, UnsideQuantity, Unsided, N1, P1, Z0};
use core::ops::{Div, Mul, Rem};

pub trait LegQuantities: Default + Sized + PartialEq + PartialOrd + Clone + Copy {
    // Basic quantities
    type Lots: QuantityOps
        + From<u64>
        + Mul<Self::AtomsPerLot, Output = Self::Atoms>
        + TryIntoUnsidedDelta<Self, P1, Z0, Z0>
        + UnsideQuantity<Self, Output = Unsided<P1, Z0, Z0, u64>>;
    type Units: QuantityOps;
    type Atoms: QuantityOps + TryIntoUnsidedDelta<Self, Z0, Z0, P1>;

    // Ratios
    type LotsPerUnit: QuantityOps;

    type AtomsPerUnit: QuantityOps
        + Rem<Self::LotsPerUnit, Output = Self::AtomsPerUnit>
        + Div<Self::LotsPerUnit, Output = Self::AtomsPerLot>;
    type AtomsPerLot: QuantityOps + TryIntoUnsidedDelta<Self, N1, Z0, P1>;
}
