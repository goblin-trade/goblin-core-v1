use crate::{
    axis::leg::{leg_constants::LegConstants, Base},
    quantities::{BaseAtomsPerBaseUnit, Position, DELA_ATOMS_PER_UNIT},
};

impl LegConstants for Base {
    const ATOMS_PER_UNIT: Self::AtomsPerUnit =
        BaseAtomsPerBaseUnit::new(DELA_ATOMS_PER_UNIT.inner as u64);
    const DEFAULT_PRICE_LIMIT: Position = Position::ZERO;
}
