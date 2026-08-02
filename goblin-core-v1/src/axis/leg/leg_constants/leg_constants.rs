use crate::{axis::leg::leg_quantities::LegQuantities, quantities::Position};

pub trait LegConstants: LegQuantities {
    const ATOMS_PER_UNIT: Self::AtomsPerUnit;
    const DEFAULT_PRICE_LIMIT: Position;
}
