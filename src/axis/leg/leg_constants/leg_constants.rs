use crate::{axis::leg::leg_quantities::LegQuantities, quantities::Ticks};

pub trait LegConstants: LegQuantities {
    const ATOMS_PER_UNIT: Self::AtomsPerUnit;
    const DEFAULT_PRICE_LIMIT: Ticks;
}
