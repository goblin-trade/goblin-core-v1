use crate::{quantities::Ticks, types::LegQuantities};

pub trait LegConstants: LegQuantities {
    const ATOMS_PER_UNIT: Self::AtomsPerUnit;
    const DEFAULT_PRICE_LIMIT: Ticks;
}
