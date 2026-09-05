use crate::{axis::leg::LegQuantities, quantities::Position};

pub trait LegConstants: LegQuantities {
    const ATOMS_PER_UNIT: Self::AtomsPerUnit;
    const DEFAULT_PRICE_LIMIT: Position;
}
