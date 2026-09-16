mod base;
mod quote;

use crate::{
    axis::leg::LegQuantities,
    quantities::{FullPosU32, U32Variant},
};

pub trait LegConstants: LegQuantities {
    const ATOMS_PER_UNIT: U32Variant<Self::AtomsPerUnit>;
    const DEFAULT_PRICE_LIMIT: FullPosU32;
}
