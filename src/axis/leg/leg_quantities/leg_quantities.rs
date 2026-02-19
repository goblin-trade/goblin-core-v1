use crate::quantities::{AsUnsided, QuantityOps, P1, Z0};
use core::ops::{Div, Mul, Rem};

pub trait LegQuantities: Sized + PartialEq + PartialOrd {
    // Basic quantities
    type Lots: QuantityOps + From<u64> + PartialOrd + Mul<Self::AtomsPerLot, Output = Self::Atoms>;
    type Units: QuantityOps;
    type Atoms: QuantityOps + AsUnsided<Self, Z0, Z0, P1>;

    // Ratios
    type LotsPerUnit: QuantityOps;
    type AtomsPerUnit: QuantityOps
        + Rem<Self::LotsPerUnit, Output = Self::AtomsPerUnit>
        + Div<Self::LotsPerUnit, Output = Self::AtomsPerLot>;
    type AtomsPerLot: QuantityOps;
}
