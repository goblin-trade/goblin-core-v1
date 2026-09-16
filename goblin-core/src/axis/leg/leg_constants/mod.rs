mod base;
mod quote;

use crate::{axis::leg::LegQuantities, quantities::FullPosU32};

pub trait LegConstants: LegQuantities {
    const ATOMS_PER_UNIT: Self::AtomsPerUnit;
    const DEFAULT_PRICE_LIMIT: FullPosU32;
}
